use crate::display_mode::{
    iterate_display_modes, wrap_display_mode, DecklinkDisplayMode, DecklinkDisplayModeId,
};
use crate::frame::{
    unwrap_frame, wrap_frame, wrap_mutable_frame, DecklinkFrameFlags, DecklinkPixelFormat,
    DecklinkVideoFrame, DecklinkVideoMutableFrame,
};
use crate::{sdk, SdkError};
use num_traits::FromPrimitive;
use std::ptr::null_mut;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

bitflags! {
    pub struct DecklinkVideoOutputFlags: u32 {
        const VANC = sdk::_DecklinkVideoOutputFlags_decklinkVideoOutputVANC;
        const VITC = sdk::_DecklinkVideoOutputFlags_decklinkVideoOutputVITC;
        const RP188 = sdk::_DecklinkVideoOutputFlags_decklinkVideoOutputRP188;
        const DUAL_STREAM_3D = sdk::_DecklinkVideoOutputFlags_decklinkVideoOutputDualStream3D;
    }
}

#[derive(FromPrimitive, PartialEq)]
pub enum DecklinkAudioSampleRate {
    Rate48kHz = sdk::_DecklinkAudioSampleRate_decklinkAudioSampleRate48kHz as isize,
}
#[derive(FromPrimitive, PartialEq)]
pub enum DecklinkAudioSampleType {
    Int16 = sdk::_DecklinkAudioSampleType_decklinkAudioSampleType16bitInteger as isize,
    Int32 = sdk::_DecklinkAudioSampleType_decklinkAudioSampleType32bitInteger as isize,
}
#[derive(FromPrimitive, PartialEq)]
pub enum DecklinkAudioOutputStreamType {
    Continuous = sdk::_DecklinkAudioOutputStreamType_decklinkAudioOutputStreamContinuous as isize,
    ContinuousDontResample =
        sdk::_DecklinkAudioOutputStreamType_decklinkAudioOutputStreamContinuousDontResample
            as isize,
}
#[derive(FromPrimitive, PartialEq)]
pub enum DecklinkDisplayModeSupport {
    NotSupported = sdk::_DecklinkDisplayModeSupport_decklinkDisplayModeNotSupported as isize,
    Supported = sdk::_DecklinkDisplayModeSupport_decklinkDisplayModeSupported as isize,
    SupportedWithConversion =
        sdk::_DecklinkDisplayModeSupport_decklinkDisplayModeSupportedWithConversion as isize,
}

#[derive(FromPrimitive, PartialEq)]
pub enum DecklinkOutputFrameCompletionResult {
    Completed = sdk::_DecklinkOutputFrameCompletionResult_decklinkOutputFrameCompleted as isize,
    DisplayedLate =
        sdk::_DecklinkOutputFrameCompletionResult_decklinkOutputFrameDisplayedLate as isize,
    Dropped = sdk::_DecklinkOutputFrameCompletionResult_decklinkOutputFrameDropped as isize,
    Flushed = sdk::_DecklinkOutputFrameCompletionResult_decklinkOutputFrameFlushed as isize,
}

struct DecklinkOutputDevicePtr {
    dev: *mut crate::sdk::cdecklink_output_t,
    video_active: Rc<AtomicBool>,
    audio_active: Rc<AtomicBool>,
}
impl Drop for DecklinkOutputDevicePtr {
    fn drop(&mut self) {
        if !self.dev.is_null() {
            unsafe { sdk::cdecklink_output_release(self.dev) };
            self.dev = null_mut();
        }
    }
}

pub struct DecklinkOutputDevice {
    ptr: Arc<DecklinkOutputDevicePtr>,
}

pub fn wrap_device_output(ptr: *mut crate::sdk::cdecklink_output_t) -> DecklinkOutputDevice {
    DecklinkOutputDevice {
        ptr: Arc::new(DecklinkOutputDevicePtr {
            dev: ptr,
            video_active: Rc::new(AtomicBool::new(false)),
            audio_active: Rc::new(AtomicBool::new(false)),
        }),
    }
}

// TODO - this is currently a bag of methods, and it could do with some more sanity checking (eg allow schedule when video not enabled etc)
impl DecklinkOutputDevice {
    pub fn does_support_video_mode(
        &self,
        mode: DecklinkDisplayModeId,
        pixel_format: DecklinkPixelFormat,
        flags: DecklinkVideoOutputFlags,
    ) -> Result<(DecklinkDisplayModeSupport, Option<DecklinkDisplayMode>), SdkError> {
        let mut supported = sdk::_DecklinkDisplayModeSupport_decklinkDisplayModeNotSupported;
        let mut display_mode = null_mut();
        let result = unsafe {
            sdk::cdecklink_output_does_support_video_mode(
                self.ptr.dev,
                mode as u32,
                pixel_format as u32,
                flags.bits(),
                &mut supported,
                &mut display_mode,
            )
        };
        SdkError::result_or_else(
            result,
            Box::new(move || {
                let supported2 = DecklinkDisplayModeSupport::from_u32(supported)
                    .unwrap_or(DecklinkDisplayModeSupport::NotSupported);
                if display_mode.is_null() || supported2 == DecklinkDisplayModeSupport::NotSupported
                {
                    (DecklinkDisplayModeSupport::NotSupported, None)
                } else {
                    unsafe { (supported2, Some(wrap_display_mode(display_mode))) }
                }
            }),
        )
    }

    pub fn display_modes(&self) -> Result<Vec<DecklinkDisplayMode>, SdkError> {
        unsafe {
            let mut it = null_mut();
            let ok = sdk::cdecklink_output_get_display_mode_iterator(self.ptr.dev, &mut it);
            if SdkError::is_ok(ok) {
                let v = iterate_display_modes(it);
                sdk::cdecklink_display_mode_iterator_release(it);
                v
            } else {
                Err(SdkError::from(ok))
            }
        }
    }

    /* Video Output */

    unsafe fn enable_video_output_inner(
        &self,
        mode: DecklinkDisplayModeId,
        flags: DecklinkVideoOutputFlags,
    ) -> i32 {
        if self.ptr.video_active.swap(true, Ordering::Relaxed) {
            // TODO - better mode
            SdkError::ACCESSDENIED as i32
        } else {
            sdk::cdecklink_output_enable_video_output(self.ptr.dev, mode as u32, flags.bits())
        }
    }

    pub fn enable_video_output_scheduled(
        self,
        mode: DecklinkDisplayModeId,
        flags: DecklinkVideoOutputFlags,
        timescale: i64,
    ) -> Result<Box<DecklinkOutputDeviceVideoScheduled>, SdkError> {
        // TODO - this leaks on an error
        let callback_wrapper = Box::into_raw(Box::new(CallbackWrapper {
            handler: RwLock::new(None),
        }));

        let result = unsafe {
            sdk::cdecklink_output_set_scheduled_frame_completion_callback(
                self.ptr.dev,
                callback_wrapper as *mut std::ffi::c_void,
                Some(schedule_frame_completed_callback),
                Some(playback_stopped),
            )
        };
        //        let result = 0;

        match SdkError::result(result) {
            Err(e) => return Err(e),
            Ok(()) => {
                let result = unsafe { self.enable_video_output_inner(mode, flags) };
                return SdkError::result_or_else(result, || {
                    let r: Box<DecklinkOutputDeviceVideoScheduled> =
                        Box::new(DecklinkOutputDeviceVideoImpl {
                            ptr: self.ptr.clone(),
                            callback_wrapper,
                            playback_running: AtomicBool::new(false),
                            scheduled_running: false,
                            scheduled_timescale: timescale,
                        });
                    r
                });
            }
        }
    }
    pub fn enable_video_output_sync(
        &self,
        mode: DecklinkDisplayModeId,
        flags: DecklinkVideoOutputFlags,
    ) -> Result<Rc<DecklinkOutputDeviceVideoSync>, SdkError> {
        let result = unsafe { self.enable_video_output_inner(mode, flags) };
        SdkError::result_or_else(result, || {
            let r: Rc<DecklinkOutputDeviceVideoSync> = Rc::new(DecklinkOutputDeviceVideoImpl {
                ptr: self.ptr.clone(),
                callback_wrapper: null_mut(),
                playback_running: AtomicBool::new(false),
                scheduled_running: false,
                scheduled_timescale: 1000,
            });
            r
        })
    }

    pub fn create_video_frame(
        &self,
        width: i32,
        height: i32,
        row_bytes: i32,
        pixel_format: DecklinkPixelFormat,
        flags: DecklinkFrameFlags,
    ) -> Result<DecklinkVideoMutableFrame, SdkError> {
        unsafe {
            let mut frame = null_mut();
            let res = sdk::cdecklink_output_create_video_frame(
                self.ptr.dev,
                width,
                height,
                row_bytes,
                pixel_format as u32,
                flags.bits(),
                &mut frame,
            );
            if SdkError::is_ok(res) {
                Ok(wrap_mutable_frame(frame))
            } else {
                Err(SdkError::from(res))
            }
        }
    }

    /* Audio Output */

    pub fn enable_audio_output(
        &self,
        sample_rate: DecklinkAudioSampleRate,
        sample_type: DecklinkAudioSampleType,
        channels: u32,
        stream_type: DecklinkAudioOutputStreamType,
    ) -> Result<DecklinkOutputDeviceAudio, SdkError> {
        if self.ptr.audio_active.swap(true, Ordering::Relaxed) {
            // TODO - better mode
            Err(SdkError::ACCESSDENIED)
        } else {
            unsafe {
                let result = sdk::cdecklink_output_enable_audio_output(
                    self.ptr.dev,
                    sample_rate as u32,
                    sample_type as u32,
                    channels,
                    stream_type as u32,
                );
                SdkError::result_or_else(result, || DecklinkOutputDeviceAudio {
                    ptr: self.ptr.clone(),
                })
            }
        }
    }
}

pub trait DeckLinkVideoOutputCallback {
    fn schedule_frame_completed_callback(
        &self,
        frame: Option<DecklinkVideoFrame>,
        result: DecklinkOutputFrameCompletionResult,
    ) -> bool;
    fn playback_stopped(&self) -> bool;
}

struct CallbackWrapper {
    handler: RwLock<Option<Arc<DeckLinkVideoOutputCallback>>>,
}
extern "C" fn schedule_frame_completed_callback(
    context: *mut ::std::os::raw::c_void,
    frame: *mut sdk::cdecklink_video_frame_t,
    result: sdk::DecklinkOutputFrameCompletionResult,
) -> sdk::HRESULT {
    let wrapper: &mut CallbackWrapper = unsafe { &mut *(context as *mut _) };

    let mut res = true;
    if let Some(handler) = &*wrapper.handler.read().unwrap() {
        let frame_internal = if frame.is_null() {
            None
        } else {
            unsafe { Some(wrap_frame(frame)) }
        };

        let result_internal = DecklinkOutputFrameCompletionResult::from_u32(result)
            .unwrap_or(DecklinkOutputFrameCompletionResult::Completed);

        res = handler.schedule_frame_completed_callback(frame_internal, result_internal);
    }

    if res {
        0 // Ok
    } else {
        1 // False
    }
}
extern "C" fn playback_stopped(context: *mut ::std::os::raw::c_void) -> sdk::HRESULT {
    let wrapper: &mut CallbackWrapper = unsafe { &mut *(context as *mut _) };

    let mut result = true;
    if let Some(handler) = &*wrapper.handler.read().unwrap() {
        result = handler.playback_stopped();
    }

    if result {
        0 // Ok
    } else {
        1 // False
    }
}

pub trait DecklinkOutputDeviceVideo {
    // TODO - is this valid in sync context?
    fn buffered_video_frame_count(&self) -> Result<u32, SdkError>;
}
pub trait DecklinkOutputDeviceVideoScheduled: DecklinkOutputDeviceVideo {
    // TODO return type
    fn schedule_frame(
        &self,
        frame: &DecklinkVideoFrame,
        display_time: i64,
        duration: i64,
    ) -> Result<(), SdkError>;

    fn set_callback(
        &mut self,
        handler: Option<Arc<DeckLinkVideoOutputCallback>>,
    ) -> Result<(), SdkError>;

    fn start_playback(&mut self, start_time: i64, speed: f64) -> Result<(), SdkError>;
    fn stop_playback(&mut self, stop_time: i64) -> Result<i64, SdkError>;
}
pub trait DecklinkOutputDeviceVideoSync: DecklinkOutputDeviceVideo {
    // TODO return type
    fn display_frame(&self, frame: &DecklinkVideoFrame) -> Result<(), SdkError>;
}

struct DecklinkOutputDeviceVideoImpl {
    ptr: Arc<DecklinkOutputDevicePtr>,
    callback_wrapper: *mut CallbackWrapper,
    playback_running: AtomicBool,
    scheduled_running: bool,
    scheduled_timescale: i64,
}
impl Drop for DecklinkOutputDeviceVideoImpl {
    fn drop(&mut self) {
        unsafe {
            if self.scheduled_running {
                let mut actual_stop = 0;
                sdk::cdecklink_output_stop_scheduled_playback(
                    self.ptr.dev,
                    0,
                    &mut actual_stop,
                    self.scheduled_timescale,
                );
            }

            // This call blocks until all frame callbacks are complete
            sdk::cdecklink_output_disable_video_output(self.ptr.dev);
            self.ptr.video_active.store(false, Ordering::Relaxed);

            Box::from_raw(self.callback_wrapper); // Reclaim the box so it gets freed
        }
    }
}

impl DecklinkOutputDeviceVideo for DecklinkOutputDeviceVideoImpl {
    fn buffered_video_frame_count(&self) -> Result<u32, SdkError> {
        unsafe {
            let mut count = 0;
            let result =
                sdk::cdecklink_output_get_buffered_video_frame_count(self.ptr.dev, &mut count);
            SdkError::result_or(result, count)
        }
    }
}
impl DecklinkOutputDeviceVideoScheduled for DecklinkOutputDeviceVideoImpl {
    fn schedule_frame(
        &self,
        frame: &DecklinkVideoFrame,
        display_time: i64,
        duration: i64,
    ) -> Result<(), SdkError> {
        unsafe {
            let result = sdk::cdecklink_output_schedule_video_frame(
                self.ptr.dev,
                unwrap_frame(frame),
                display_time,
                duration,
                self.scheduled_timescale,
            );
            SdkError::result(result)
        }
    }

    fn set_callback(
        &mut self,
        handler: Option<Arc<DeckLinkVideoOutputCallback>>,
    ) -> Result<(), SdkError> {
        if self.callback_wrapper.is_null() {
            Err(SdkError::HANDLE)
        } else {
            unsafe {
                let wrapper = &(*self.callback_wrapper);
                *wrapper.handler.write().unwrap() = handler;
            }
            Ok(())
        }
    }

    fn start_playback(&mut self, start_time: i64, speed: f64) -> Result<(), SdkError> {
        if self.scheduled_running {
            Ok(())
        } else {
            self.scheduled_running = true;
            self.playback_running.store(true, Ordering::Relaxed); // TODO - order

            unsafe {
                let result = sdk::cdecklink_output_start_scheduled_playback(
                    self.ptr.dev,
                    start_time,
                    self.scheduled_timescale,
                    speed,
                );
                SdkError::result(result)
            }
        }
    }

    fn stop_playback(&mut self, stop_time: i64) -> Result<i64, SdkError> {
        if self.scheduled_running {
            self.scheduled_running = false;

            unsafe {
                let mut actual_stop_time = 0;
                let result = sdk::cdecklink_output_stop_scheduled_playback(
                    self.ptr.dev,
                    stop_time,
                    &mut actual_stop_time,
                    self.scheduled_timescale,
                );
                SdkError::result_or(result, actual_stop_time)
            }
        } else {
            Err(SdkError::FALSE)
        }
    }
}

impl DecklinkOutputDeviceVideoSync for DecklinkOutputDeviceVideoImpl {
    fn display_frame(&self, frame: &DecklinkVideoFrame) -> Result<(), SdkError> {
        unsafe {
            let result =
                sdk::cdecklink_output_display_video_frame_sync(self.ptr.dev, unwrap_frame(frame));
            SdkError::result(result)
        }
    }
}

pub struct DecklinkOutputDeviceAudio {
    ptr: Arc<DecklinkOutputDevicePtr>,
}
impl Drop for DecklinkOutputDeviceAudio {
    fn drop(&mut self) {
        unsafe {
            sdk::cdecklink_output_disable_audio_output(self.ptr.dev);
            self.ptr.audio_active.store(false, Ordering::Relaxed)
        }
    }
}
impl DecklinkOutputDeviceAudio {
    //    pub fn write_audio_samples_sync(&self, )
    //    HRESULT cdecklink_output_write_audio_samples_sync(cdecklink_output_t *output, void *buffer,
    //    uint32_t sampleFrameCount, uint32_t *sampleFramesWritten);

    pub fn begin_audio_preroll(&self) -> Result<(), SdkError> {
        unsafe {
            let result = sdk::cdecklink_output_begin_audio_preroll(self.ptr.dev);
            SdkError::result(result)
        }
    }
    pub fn end_audio_preroll(&self) -> Result<(), SdkError> {
        unsafe {
            let result = sdk::cdecklink_output_end_audio_preroll(self.ptr.dev);
            SdkError::result(result)
        }
    }

    //    HRESULT cdecklink_output_schedule_audio_samples(cdecklink_output_t *output, void *buffer,
    //    uint32_t sampleFrameCount, int64_t streamTime,
    //    int64_t timeScale, uint32_t *sampleFramesWritten);

    pub fn buffered_audio_sample_frame_count(&self) -> Result<u32, SdkError> {
        unsafe {
            let mut count = 0;
            let result = sdk::cdecklink_output_get_buffered_audio_sample_frame_count(
                self.ptr.dev,
                &mut count,
            );
            SdkError::result_or(result, count)
        }
    }
    pub fn flush_buffered_audio_samples(&self) -> Result<(), SdkError> {
        unsafe {
            let result = sdk::cdecklink_output_flush_buffered_audio_samples(self.ptr.dev);
            SdkError::result(result)
        }
    }
}
