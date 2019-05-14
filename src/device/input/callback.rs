use crate::device::input::audio::{wrap_packet, DecklinkAudioInputPacket};
use crate::device::input::enums::{
    DecklinkDetectedVideoInputFormatFlags, DecklinkVideoInputFormatChangedEvents,
};
use crate::display_mode::{wrap_display_mode, DecklinkDisplayMode};
use crate::frame::{wrap_frame, DecklinkVideoFrame};
use crate::{sdk, SdkError};
use std::sync::{Arc, RwLock};

pub fn free_callback_wrapper(wrapper: *mut CallbackWrapper) {
    unsafe {
        Box::from_raw(wrapper);
    }
}

pub fn register_callback(
    ptr: *mut sdk::cdecklink_input_t,
    audio_channels: u32,
    audio_sample_depth: u32,
) -> Result<*mut CallbackWrapper, SdkError> {
    let callback_wrapper = Box::into_raw(Box::new(CallbackWrapper {
        handler: RwLock::new(None),
        audio_channels,
        audio_sample_depth,
    }));

    let result = unsafe {
        sdk::cdecklink_input_set_callback(
            ptr,
            callback_wrapper as *mut std::ffi::c_void,
            Some(video_input_format_changed),
            Some(input_frame_arrived),
        )
    };

    match SdkError::result_or(result, callback_wrapper) {
        Err(e) => {
            free_callback_wrapper(callback_wrapper);
            Err(e)
        }
        Ok(v) => Ok(v),
    }
}

pub trait DeckLinkVideoInputCallback {
    fn input_frame_arrived(
        &self,
        video_frame: Option<DecklinkVideoFrame>,
        audio_packet: Option<DecklinkAudioInputPacket>,
    ) -> bool;
    fn video_input_format_changed(
        &self,
        events: DecklinkVideoInputFormatChangedEvents,
        display_mode: Option<DecklinkDisplayMode>,
        signal_flags: DecklinkDetectedVideoInputFormatFlags,
    ) -> bool;
}

pub struct CallbackWrapper {
    pub handler: RwLock<Option<Arc<DeckLinkVideoInputCallback>>>,
    pub audio_channels: u32,
    pub audio_sample_depth: u32,
}
extern "C" fn input_frame_arrived(
    context: *mut ::std::os::raw::c_void,
    video_frame: *mut sdk::cdecklink_video_input_frame_t,
    audio_packet: *mut sdk::cdecklink_audio_input_packet_t,
) -> sdk::HRESULT {
    let wrapper: &mut CallbackWrapper = unsafe { &mut *(context as *mut _) };

    let mut res = true;
    if let Some(handler) = &*wrapper.handler.read().unwrap() {
        let frame_internal = if video_frame.is_null() {
            None
        } else {
            unsafe { Some(wrap_frame(video_frame)) }
        };

        let packet_internal = if audio_packet.is_null() {
            None
        } else {
            unsafe {
                Some(wrap_packet(
                    audio_packet,
                    wrapper.audio_channels,
                    wrapper.audio_sample_depth,
                ))
            }
        };

        res = handler.input_frame_arrived(frame_internal, packet_internal);
    }

    if res {
        0 // Ok
    } else {
        1 // False
    }
}
extern "C" fn video_input_format_changed(
    context: *mut ::std::os::raw::c_void,
    events: sdk::DecklinkVideoInputFormatChangedEvents,
    new_display_mode: *mut sdk::cdecklink_display_mode_t,
    detected_signal_flags: sdk::DecklinkDetectedVideoInputFormatFlags,
) -> sdk::HRESULT {
    let wrapper: &mut CallbackWrapper = unsafe { &mut *(context as *mut _) };

    let mut result = true;
    if let Some(handler) = &*wrapper.handler.read().unwrap() {
        let display_mode = if new_display_mode.is_null() {
            None
        } else {
            unsafe { Some(wrap_display_mode(new_display_mode)) }
        };
        result = handler.video_input_format_changed(
            DecklinkVideoInputFormatChangedEvents::from_bits_truncate(events),
            display_mode,
            DecklinkDetectedVideoInputFormatFlags::from_bits_truncate(detected_signal_flags),
        );
    }

    if result {
        0 // Ok
    } else {
        1 // False
    }
}
