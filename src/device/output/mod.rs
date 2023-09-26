mod audio;
mod device;
mod enums;
mod video;
mod video_callback;

use crate::device::output::device::DecklinkOutputDevicePtr;
use crate::device::output::video_callback::register_callback;
use crate::display_mode::{
    iterate_display_modes, wrap_display_mode, DecklinkDisplayMode, DecklinkDisplayModeId,
};
use crate::frame::DecklinkPixelFormat;
use crate::{sdk, SdkError};
use num_traits::FromPrimitive;
use std::ptr::null_mut;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

pub use crate::device::output::audio::DecklinkOutputDeviceAudio;
pub use crate::device::output::enums::*;
pub use crate::device::output::video::{
    DecklinkOutputDeviceVideoScheduled, DecklinkOutputDeviceVideoSync,
};
pub use crate::device::output::video_callback::DeckLinkVideoOutputCallback;
use crate::device::{DecklinkDeviceDisplayModes, DecklinkDisplayModeSupport};

use self::video::DecklinkOutputDeviceVideoImpl;

pub struct DecklinkOutputDevice {
    ptr: Rc<DecklinkOutputDevicePtr>,
}

impl DecklinkDeviceDisplayModes<enums::DecklinkVideoOutputFlags> for DecklinkOutputDevice {
    fn does_support_video_mode(
        &self,
        mode: DecklinkDisplayModeId,
        pixel_format: DecklinkPixelFormat,
        flags: enums::DecklinkVideoOutputFlags,
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
        SdkError::result_or_else(result, move || {
            let supported2 = DecklinkDisplayModeSupport::from_u32(supported)
                .unwrap_or(DecklinkDisplayModeSupport::NotSupported);
            if display_mode.is_null() || supported2 == DecklinkDisplayModeSupport::NotSupported {
                (DecklinkDisplayModeSupport::NotSupported, None)
            } else {
                unsafe { (supported2, Some(wrap_display_mode(display_mode))) }
            }
        })
    }

    fn display_modes(&self) -> Result<Vec<DecklinkDisplayMode>, SdkError> {
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
}
// TODO - this is currently a bag of methods, and it could do with some more sanity checking (eg allow schedule when video not enabled etc)
impl DecklinkOutputDevice {
    pub(crate) fn from(ptr: *mut crate::sdk::cdecklink_output_t) -> DecklinkOutputDevice {
        DecklinkOutputDevice {
            ptr: Rc::new(DecklinkOutputDevicePtr {
                dev: ptr,
                video_active: Rc::new(AtomicBool::new(false)),
                audio_active: Rc::new(AtomicBool::new(false)),
            }),
        }
    }

    /* Video Output */

    unsafe fn enable_video_output_inner(
        &self,
        mode: DecklinkDisplayModeId,
        flags: enums::DecklinkVideoOutputFlags,
    ) -> i32 {
        if self.ptr.video_active.swap(true, Ordering::Relaxed) {
            // TODO - better mode
            SdkError::ACCESSDENIED as i32
        } else {
            sdk::cdecklink_output_enable_video_output(self.ptr.dev, mode as u32, flags.bits())
        }
    }

    pub fn is_scheduled_playback_running(&self) -> Result<bool, SdkError> {
        unsafe {
            let mut running = false;
            let result =
                sdk::cdecklink_output_is_scheduled_playback_running(self.ptr.dev, &mut running);
            SdkError::result_or(result, running)
        }
    }

    pub fn enable_video_output_scheduled(
        &self,
        mode: DecklinkDisplayModeId,
        flags: enums::DecklinkVideoOutputFlags,
        timescale: i64,
    ) -> Result<Box<dyn DecklinkOutputDeviceVideoScheduled>, SdkError> {
        match register_callback(&self.ptr) {
            // Don't do this if already running?
            Err(e) => Err(e),
            Ok(wrapper) => {
                // TODO - this leaks on error
                let result = unsafe { self.enable_video_output_inner(mode, flags) };
                SdkError::result_or_else(result, || {
                    let r: Box<dyn DecklinkOutputDeviceVideoScheduled> = Box::new(
                        DecklinkOutputDeviceVideoImpl::from(&self.ptr, wrapper, timescale),
                    );
                    r
                })
            }
        }
    }
    pub fn enable_video_output_sync(
        &self,
        mode: DecklinkDisplayModeId,
        flags: enums::DecklinkVideoOutputFlags,
    ) -> Result<Box<dyn DecklinkOutputDeviceVideoSync>, SdkError> {
        let result = unsafe { self.enable_video_output_inner(mode, flags) };
        SdkError::result_or_else(result, || {
            let r: Box<dyn DecklinkOutputDeviceVideoSync> = Box::new(
                DecklinkOutputDeviceVideoImpl::from(&self.ptr, null_mut(), 1000),
            );
            r
        })
    }

    /* Audio Output */

    pub fn enable_audio_output(
        &self,
        sample_rate: enums::DecklinkAudioSampleRate,
        sample_type: enums::DecklinkAudioSampleType,
        channels: u32,
        stream_type: enums::DecklinkAudioOutputStreamType,
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
                SdkError::result_or_else(result, || DecklinkOutputDeviceAudio::from(&self.ptr))
            }
        }
    }
}
