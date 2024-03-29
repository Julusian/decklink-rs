use crate::device::output::video_callback::{CallbackWrapper, DeckLinkVideoOutputCallback};
use crate::device::output::DecklinkOutputDevicePtr;
use crate::frame::{DecklinkAlignedVec, DecklinkFrameBase, DecklinkFrameBase2};
use crate::{sdk, SdkError};
use std::ptr::null_mut;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub trait DecklinkOutputDeviceVideo {}
pub trait DecklinkOutputDeviceVideoSync: DecklinkOutputDeviceVideo {
    // TODO return type
    fn display_frame_copy(&self, frame: &dyn DecklinkFrameBase) -> Result<(), SdkError>;
    // TODO return type
    fn display_custom_frame(&self, frame: Box<dyn DecklinkFrameBase2>) -> Result<(), SdkError>;
}
pub trait DecklinkOutputDeviceVideoScheduled: DecklinkOutputDeviceVideo {
    // TODO return type
    fn schedule_frame_copy(
        &self,
        frame: &dyn DecklinkFrameBase,
        display_time: i64,
        duration: i64,
    ) -> Result<(), SdkError>;

    fn set_callback(
        &mut self,
        handler: Option<Arc<dyn DeckLinkVideoOutputCallback>>,
    ) -> Result<(), SdkError>;

    fn buffered_video_frame_count(&self) -> Result<u32, SdkError>;

    fn start_playback(&mut self, start_time: i64, speed: f64) -> Result<(), SdkError>;
    fn stop_playback(&mut self, stop_time: i64) -> Result<i64, SdkError>;
}

pub(crate) struct DecklinkOutputDeviceVideoImpl {
    ptr: Rc<DecklinkOutputDevicePtr>,
    pub callback_wrapper: *mut CallbackWrapper,
    pub scheduled_running: bool,
    pub scheduled_timescale: i64,
}
impl Drop for DecklinkOutputDeviceVideoImpl {
    fn drop(&mut self) {
        // TODO - safety!
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

            if !self.callback_wrapper.is_null() {
                drop(Box::from_raw(self.callback_wrapper)); // Reclaim the box so it gets freed
                self.callback_wrapper = null_mut();
            }
        }
    }
}

impl DecklinkOutputDeviceVideo for DecklinkOutputDeviceVideoImpl {}

impl DecklinkOutputDeviceVideoSync for DecklinkOutputDeviceVideoImpl {
    fn display_frame_copy(&self, frame: &dyn DecklinkFrameBase) -> Result<(), SdkError> {
        let decklink_frame = self.convert_decklink_frame_without_bytes(frame)?;

        let mut ptr = std::ptr::null_mut();
        let result = unsafe { sdk::cdecklink_video_frame_get_bytes(decklink_frame.ptr, &mut ptr) };
        SdkError::result(result)?;

        let byte_count = frame.row_bytes() * frame.height();
        let src_bytes = frame.bytes()?;
        if src_bytes.0.len() < byte_count {
            Err(SdkError::INVALIDARG)?;
        }
        unsafe { std::ptr::copy(src_bytes.0.as_ptr(), ptr as *mut _, byte_count) };

        let result = unsafe {
            sdk::cdecklink_output_display_video_frame_sync(self.ptr.dev, decklink_frame.ptr)
        };

        SdkError::result(result)
    }

    fn display_custom_frame(&self, frame: Box<dyn DecklinkFrameBase2>) -> Result<(), SdkError> {
        let mut decklink_frame = WrappedCustomFrame { ptr: null_mut() };
        let result = unsafe {
            sdk::cdecklink_custom_video_frame_create_frame(
                frame.width() as i64,
                frame.height() as i64,
                frame.row_bytes() as i64,
                frame.pixel_format() as u32,
                frame.flags().bits(),
                &mut decklink_frame.ptr,
            )
        };
        SdkError::result(result)?;

        if decklink_frame.ptr.is_null() {
            Err(SdkError::FAIL)?;
        }

        let required_bytes = frame.row_bytes() * frame.height();
        let bytes = frame.into_avec()?;
        if bytes.len() < required_bytes {
            Err(SdkError::INVALIDARG)?;
        }
        if required_bytes % 64 != 0 {
            // Must be a multiple of 64 to be valid for avx512
            Err(SdkError::INVALIDARG)?;
        }

        let context = LeakableVec::from(bytes);

        unsafe {
            sdk::cdecklink_custom_video_frame_set_bytes(
                decklink_frame.ptr,
                context.get_data_ptr(),
                Some(free_vec),
                context.into_raw_ptr(),
            )
        };

        let result = unsafe {
            sdk::cdecklink_output_display_video_frame_sync(self.ptr.dev, decklink_frame.ptr)
        };

        SdkError::result(result)
    }
}

impl DecklinkOutputDeviceVideoScheduled for DecklinkOutputDeviceVideoImpl {
    fn schedule_frame_copy(
        &self,
        frame: &dyn DecklinkFrameBase,
        display_time: i64,
        duration: i64,
    ) -> Result<(), SdkError> {
        let frame_bytes = frame.bytes()?;
        let byte_count = frame.row_bytes() * frame.height();
        if frame_bytes.0.len() < byte_count {
            Err(SdkError::INVALIDARG)?;
        }

        let frame = self.convert_decklink_frame_without_bytes(frame)?;

        let mut bytes_ptr = std::ptr::null_mut();
        let result = unsafe { sdk::cdecklink_video_frame_get_bytes(frame.ptr, &mut bytes_ptr) };
        SdkError::result(result)?;
        if bytes_ptr.is_null() {
            Err(SdkError::FAIL)?;
        }

        unsafe { std::ptr::copy(frame_bytes.0.as_ptr(), bytes_ptr as *mut _, byte_count) };

        let result = unsafe {
            sdk::cdecklink_output_schedule_video_frame(
                self.ptr.dev,
                frame.ptr,
                display_time,
                duration,
                self.scheduled_timescale,
            )
        };
        SdkError::result(result)
    }

    fn set_callback(
        &mut self,
        handler: Option<Arc<dyn DeckLinkVideoOutputCallback>>,
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

    fn buffered_video_frame_count(&self) -> Result<u32, SdkError> {
        unsafe {
            let mut count = 0;
            let result =
                sdk::cdecklink_output_get_buffered_video_frame_count(self.ptr.dev, &mut count);
            SdkError::result_or(result, count)
        }
    }

    fn start_playback(&mut self, start_time: i64, speed: f64) -> Result<(), SdkError> {
        if self.scheduled_running {
            Ok(())
        } else {
            self.scheduled_running = true;

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

impl DecklinkOutputDeviceVideoImpl {
    pub(crate) fn from(
        ptr: &Rc<DecklinkOutputDevicePtr>,
        wrapper: *mut CallbackWrapper,
        timescale: i64,
    ) -> DecklinkOutputDeviceVideoImpl {
        DecklinkOutputDeviceVideoImpl {
            ptr: ptr.clone(),
            callback_wrapper: wrapper,
            scheduled_running: false,
            scheduled_timescale: timescale,
        }
    }

    pub(crate) fn convert_decklink_frame_without_bytes(
        &self,
        frame: &dyn DecklinkFrameBase,
    ) -> Result<WrappedSdkFrame, SdkError> {
        let mut c_frame = null_mut();
        unsafe {
            let res = sdk::cdecklink_output_create_video_frame(
                self.ptr.dev,
                frame.width() as i32,
                frame.height() as i32,
                frame.row_bytes() as i32,
                frame.pixel_format() as u32,
                frame.flags().bits(),
                &mut c_frame,
            );
            SdkError::result(res)?;

            if c_frame.is_null() {
                Err(SdkError::FAIL)?;
            }

            sdk::cdecklink_video_frame_add_ref(c_frame);
        }

        Ok(WrappedSdkFrame { ptr: c_frame })
    }
}

pub(crate) struct WrappedSdkFrame {
    pub ptr: *mut crate::sdk::cdecklink_mutable_video_frame_t,
}
impl Drop for WrappedSdkFrame {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                sdk::cdecklink_video_frame_release(self.ptr);
            }
            self.ptr = null_mut();
        }
    }
}

pub(crate) struct WrappedCustomFrame {
    pub ptr: *mut crate::sdk::cdecklink_custom_video_frame_t,
}
impl Drop for WrappedCustomFrame {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                sdk::cdecklink_custom_video_frame_release(self.ptr);
            }
            self.ptr = null_mut();
        }
    }
}

struct LeakableVec {
    vec: DecklinkAlignedVec,
}
trait LeakableVec2 {
    fn into_raw_ptr(self) -> *mut ::std::os::raw::c_void;
    fn get_data_ptr(&self) -> *mut ::std::os::raw::c_void;
}
impl LeakableVec2 for Box<LeakableVec> {
    fn into_raw_ptr(self) -> *mut ::std::os::raw::c_void {
        Box::<LeakableVec>::into_raw(self) as *mut _
    }
    fn get_data_ptr(&self) -> *mut ::std::os::raw::c_void {
        self.vec.as_ptr() as *mut _
    }
}
impl LeakableVec {
    pub fn from(vec: DecklinkAlignedVec) -> Box<LeakableVec> {
        Box::new(LeakableVec { vec })
    }
}

unsafe extern "C" fn free_vec(
    _ptr: *mut ::std::os::raw::c_void,
    context: *mut ::std::os::raw::c_void,
) {
    let wrapper = Box::from_raw(context as *mut LeakableVec);

    drop(wrapper);
}
