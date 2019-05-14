mod audio;
mod callback;
mod enums;

use crate::device::common::{DecklinkAudioSampleRate, DecklinkAudioSampleType};
use crate::device::input::callback::{register_callback, CallbackWrapper};
use crate::device::{DecklinkDeviceDisplayModes, DecklinkDisplayModeSupport};
use crate::display_mode::{
    iterate_display_modes, wrap_display_mode, DecklinkDisplayMode, DecklinkDisplayModeId,
};
use crate::frame::DecklinkPixelFormat;
use crate::{sdk, SdkError};
use num_traits::FromPrimitive;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};

pub use crate::device::input::audio::DecklinkAudioInputPacket;
pub use crate::device::input::callback::DeckLinkVideoInputCallback;
pub use crate::device::input::enums::*;

pub struct DecklinkInputDevice {
    dev: *mut crate::sdk::cdecklink_input_t,
    // TODO - should these really have locks?
    video_enabled: Mutex<bool>,
    audio_enabled: Mutex<bool>,
    stream_status: Mutex<StreamStatus>,

    callback_wrapper: *mut CallbackWrapper,
}

enum StreamStatus {
    Stopped,
    Running,
    Paused,
}

impl Drop for DecklinkInputDevice {
    fn drop(&mut self) {
        unsafe {
            sdk::cdecklink_encoder_input_stop_streams(self.dev);
            // TODO - disable video/audio?

            sdk::cdecklink_input_release(self.dev);

            Box::from_raw(self.callback_wrapper); // Reclaim the box so it gets freed
        }
    }
}

pub fn wrap_device_input(ptr: *mut crate::sdk::cdecklink_input_t) -> DecklinkInputDevice {
    DecklinkInputDevice {
        dev: ptr,
        video_enabled: Mutex::new(false),
        audio_enabled: Mutex::new(false),
        stream_status: Mutex::new(StreamStatus::Stopped),
        callback_wrapper: null_mut(),
    }
}

impl DecklinkDeviceDisplayModes<enums::DecklinkVideoInputFlags> for DecklinkInputDevice {
    fn does_support_video_mode(
        &self,
        mode: DecklinkDisplayModeId,
        pixel_format: DecklinkPixelFormat,
        flags: enums::DecklinkVideoInputFlags,
    ) -> Result<(DecklinkDisplayModeSupport, Option<DecklinkDisplayMode>), SdkError> {
        let mut supported = sdk::_DecklinkDisplayModeSupport_decklinkDisplayModeNotSupported;
        let mut display_mode = null_mut();
        let result = unsafe {
            sdk::cdecklink_input_does_support_video_mode(
                self.dev,
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
            let ok = sdk::cdecklink_input_get_display_mode_iterator(self.dev, &mut it);
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

// TODO - these need some rewriting to make it simpler and safer to use
impl DecklinkInputDevice {
    /* Video Input */

    pub fn enable_video_input(
        &self,
        display_mode: DecklinkDisplayModeId,
        pixel_format: DecklinkPixelFormat,
        flags: DecklinkVideoInputFlags,
    ) -> Result<(), SdkError> {
        if let Ok(mut video) = self.video_enabled.lock() {
            let result = unsafe {
                sdk::cdecklink_input_enable_video_input(
                    self.dev,
                    display_mode as u32,
                    pixel_format as u32,
                    flags.bits(),
                )
            };
            if SdkError::is_ok(result) {
                *video = true
            }
            SdkError::result(result)
        } else {
            Err(SdkError::FAIL)
        }
    }
    pub fn disable_video_input(&self) -> Result<(), SdkError> {
        if let Ok(mut video) = self.video_enabled.lock() {
            if !*video {
                Ok(()) // ?
            } else {
                let result = unsafe { sdk::cdecklink_input_disable_video_input(self.dev) };
                if SdkError::is_ok(result) {
                    *video = false
                }
                SdkError::result(result)
            }
        } else {
            Err(SdkError::FAIL)
        }
    }

    pub fn get_available_video_frame_count(&self) -> Result<u32, SdkError> {
        if let Ok(video) = self.audio_enabled.lock() {
            if !*video {
                Ok(0)
            } else {
                let mut count = 0;
                let result = unsafe {
                    sdk::cdecklink_input_get_available_video_frame_count(self.dev, &mut count)
                };
                SdkError::result_or(result, count)
            }
        } else {
            Err(SdkError::FAIL)
        }
    }

    /* Audio Input */

    pub fn enable_audio_input(
        &self,
        sample_rate: DecklinkAudioSampleRate,
        sample_type: DecklinkAudioSampleType,
        channel_count: u32,
    ) -> Result<(), SdkError> {
        if let Ok(mut audio) = self.audio_enabled.lock() {
            if *audio {
                Ok(()) // ?
            } else {
                let result = unsafe {
                    sdk::cdecklink_input_enable_audio_input(
                        self.dev,
                        sample_rate as u32,
                        sample_type as u32,
                        channel_count,
                    )
                };
                if SdkError::is_ok(result) {
                    *audio = true
                }

                // TODO - store the channel_count & sample_type for use in calculating packet size

                SdkError::result(result)
            }
        } else {
            Err(SdkError::FAIL)
        }
    }
    pub fn disable_audio_input(&self) -> Result<(), SdkError> {
        if let Ok(mut audio) = self.audio_enabled.lock() {
            if !*audio {
                Ok(()) // ?
            } else {
                let result = unsafe { sdk::cdecklink_input_disable_audio_input(self.dev) };
                if SdkError::is_ok(result) {
                    *audio = true
                }
                SdkError::result(result)
            }
        } else {
            Err(SdkError::FAIL)
        }
    }

    pub fn get_available_audio_sample_frame_count(&self) -> Result<u32, SdkError> {
        if let Ok(audio) = self.audio_enabled.lock() {
            if !*audio {
                Ok(0)
            } else {
                let mut count = 0;
                let result = unsafe {
                    sdk::cdecklink_encoder_input_get_available_audio_sample_frame_count(
                        self.dev, &mut count,
                    )
                };
                SdkError::result_or(result, count)
            }
        } else {
            Err(SdkError::FAIL)
        }
    }

    /* Streams */

    pub fn start_streams(&self) -> Result<(), SdkError> {
        if let Ok(mut status) = self.stream_status.lock() {
            match *status {
                StreamStatus::Running => Ok(()), // Already running
                _ => {
                    let result = unsafe { sdk::cdecklink_input_start_streams(self.dev) };
                    if SdkError::is_ok(result) {
                        *status = StreamStatus::Running;
                    }
                    SdkError::result(result)
                }
            }
        } else {
            Err(SdkError::FAIL)
        }
    }

    pub fn stop_streams(&self) -> Result<(), SdkError> {
        if let Ok(mut status) = self.stream_status.lock() {
            match *status {
                StreamStatus::Stopped => Ok(()), // Already stopped
                _ => {
                    let result = unsafe { sdk::cdecklink_input_stop_streams(self.dev) };
                    if SdkError::is_ok(result) {
                        *status = StreamStatus::Stopped;
                    }
                    SdkError::result(result)
                }
            }
        } else {
            Err(SdkError::FAIL)
        }
    }

    pub fn pause_streams(&self) -> Result<(), SdkError> {
        if let Ok(mut status) = self.stream_status.lock() {
            match *status {
                StreamStatus::Stopped => Ok(()), // Nothing to do
                _ => {
                    let result = unsafe { sdk::cdecklink_input_pause_streams(self.dev) };
                    if SdkError::is_ok(result) {
                        match *status {
                            StreamStatus::Running => *status = StreamStatus::Paused,
                            StreamStatus::Paused => *status = StreamStatus::Running,
                            StreamStatus::Stopped => {}
                        }
                    }
                    SdkError::result(result)
                }
            }
        } else {
            Err(SdkError::FAIL)
        }
    }

    pub fn flush_streams(&self) -> Result<(), SdkError> {
        if let Ok(status) = self.stream_status.lock() {
            match *status {
                StreamStatus::Stopped => Ok(()), // Nothing to do
                _ => {
                    let result = unsafe { sdk::cdecklink_input_flush_streams(self.dev) };
                    SdkError::result(result)
                }
            }
        } else {
            Err(SdkError::FAIL)
        }
    }

    /* Callback */

    pub fn set_callback(
        &mut self,
        handler: Arc<DeckLinkVideoInputCallback>,
    ) -> Result<(), SdkError> {
        unsafe {
            if self.callback_wrapper.is_null() {
                match register_callback(self.dev, 1, 1) {
                    // TODO - real values
                    Ok(wrapper) => self.callback_wrapper = wrapper,
                    Err(e) => return Err(e),
                }
            }

            if let Ok(mut wrapper) = (*self.callback_wrapper).handler.write() {
                *wrapper = Some(handler);

                Ok(())
            } else {
                Err(SdkError::FAIL)
            }
        }
    }

    // IDeckLinkInput::GetHardwareReferenceClock
}
