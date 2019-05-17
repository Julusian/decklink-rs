mod audio;
mod callback;
mod enums;

use crate::device::common::DecklinkAudioSampleType;
use crate::device::input::callback::{register_callback, CallbackWrapper};
use crate::device::{DecklinkDeviceDisplayModes, DecklinkDisplayModeSupport};
use crate::display_mode::{
    iterate_display_modes, wrap_display_mode, DecklinkDisplayMode, DecklinkDisplayModeId,
};
use crate::frame::DecklinkPixelFormat;
use crate::{sdk, SdkError};
use num_traits::FromPrimitive;
use std::ptr::null_mut;
use std::sync::Arc;

pub use crate::device::input::audio::DecklinkAudioInputPacket;
pub use crate::device::input::callback::DeckLinkVideoInputCallback;
pub use crate::device::input::enums::*;

pub struct DecklinkInputDevice {
    dev: *mut crate::sdk::cdecklink_input_t,
    // TODO - should these really have locks?
    video_enabled: bool,
    audio_enabled: bool,
    stream_status: StreamStatus,

    audio_channel_count: u32,
    audio_sample_depth: u32,

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

            if !self.callback_wrapper.is_null() {
                Box::from_raw(self.callback_wrapper); // Reclaim the box so it gets freed
            }
        }
    }
}

pub fn wrap_device_input(ptr: *mut crate::sdk::cdecklink_input_t) -> DecklinkInputDevice {
    DecklinkInputDevice {
        dev: ptr,
        video_enabled: false,
        audio_enabled: false,
        stream_status: StreamStatus::Stopped,

        audio_channel_count: 2,
        audio_sample_depth: 16,

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
        &mut self,
        display_mode: DecklinkDisplayModeId,
        pixel_format: DecklinkPixelFormat,
        flags: DecklinkVideoInputFlags,
    ) -> Result<(), SdkError> {
        let result = unsafe {
            sdk::cdecklink_input_enable_video_input(
                self.dev,
                display_mode as u32,
                pixel_format as u32,
                flags.bits(),
            )
        };
        if SdkError::is_ok(result) {
            self.video_enabled = true
        }
        SdkError::result(result)
    }
    pub fn disable_video_input(&mut self) -> Result<(), SdkError> {
        if !self.video_enabled {
            Ok(()) // ?
        } else {
            let result = unsafe { sdk::cdecklink_input_disable_video_input(self.dev) };
            if SdkError::is_ok(result) {
                self.video_enabled = false
            }
            SdkError::result(result)
        }
    }

    pub fn get_available_video_frame_count(&self) -> Result<u32, SdkError> {
        if !self.video_enabled {
            Ok(0)
        } else {
            let mut count = 0;
            let result = unsafe {
                sdk::cdecklink_input_get_available_video_frame_count(self.dev, &mut count)
            };
            SdkError::result_or(result, count)
        }
    }

    /* Audio Input */

    pub fn enable_audio_input(
        &mut self,
        sample_type: DecklinkAudioSampleType,
        channel_count: u32,
    ) -> Result<(), SdkError> {
        if self.audio_enabled {
            Ok(()) // ?
        } else {
            let result = unsafe {
                sdk::cdecklink_input_enable_audio_input(
                    self.dev,
                    sdk::_DecklinkAudioSampleRate_decklinkAudioSampleRate48kHz,
                    sample_type as u32,
                    channel_count,
                )
            };
            if SdkError::is_ok(result) {
                self.audio_enabled = true;
                self.audio_channel_count = channel_count;
                self.audio_sample_depth = sample_type as u32;
            }

            SdkError::result(result)
        }
    }
    pub fn disable_audio_input(&mut self) -> Result<(), SdkError> {
        if !self.audio_enabled {
            Ok(()) // ?
        } else {
            let result = unsafe { sdk::cdecklink_input_disable_audio_input(self.dev) };
            if SdkError::is_ok(result) {
                self.audio_enabled = true
            }
            SdkError::result(result)
        }
    }

    pub fn get_available_audio_sample_frame_count(&self) -> Result<u32, SdkError> {
        if !self.audio_enabled {
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
    }

    /* Streams */

    pub fn start_streams(&mut self) -> Result<(), SdkError> {
        match self.stream_status {
            StreamStatus::Running => Ok(()), // Already running
            _ => {
                let result = unsafe { sdk::cdecklink_input_start_streams(self.dev) };
                if SdkError::is_ok(result) {
                    self.stream_status = StreamStatus::Running;
                }
                SdkError::result(result)
            }
        }
    }

    pub fn stop_streams(&mut self) -> Result<(), SdkError> {
        match self.stream_status {
            StreamStatus::Stopped => Ok(()), // Already stopped
            _ => {
                let result = unsafe { sdk::cdecklink_input_stop_streams(self.dev) };
                if SdkError::is_ok(result) {
                    self.stream_status = StreamStatus::Stopped;
                }
                SdkError::result(result)
            }
        }
    }

    pub fn pause_streams(&mut self) -> Result<(), SdkError> {
        match self.stream_status {
            StreamStatus::Stopped => Ok(()), // Nothing to do
            _ => {
                let result = unsafe { sdk::cdecklink_input_pause_streams(self.dev) };
                if SdkError::is_ok(result) {
                    match self.stream_status {
                        StreamStatus::Running => self.stream_status = StreamStatus::Paused,
                        StreamStatus::Paused => self.stream_status = StreamStatus::Running,
                        StreamStatus::Stopped => {}
                    }
                }
                SdkError::result(result)
            }
        }
    }

    pub fn flush_streams(&mut self) -> Result<(), SdkError> {
        match self.stream_status {
            StreamStatus::Stopped => Ok(()), // Nothing to do
            _ => {
                let result = unsafe { sdk::cdecklink_input_flush_streams(self.dev) };
                SdkError::result(result)
            }
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
