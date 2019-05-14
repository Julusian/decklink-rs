use crate::{sdk, SdkError};
use std::ptr::null_mut;
use std::slice;

pub unsafe fn wrap_packet(
    ptr: *mut sdk::cdecklink_video_frame_t,
    channels: u32,
    sample_depth: u32,
) -> DecklinkAudioInputPacket {
    sdk::cdecklink_audio_input_packet_add_ref(ptr);
    DecklinkAudioInputPacket {
        packet: ptr,
        channels,
        sample_depth,
    }
}

pub struct DecklinkAudioInputPacket {
    packet: *mut crate::sdk::cdecklink_audio_input_packet_t,
    channels: u32,
    sample_depth: u32,
}

impl Drop for DecklinkAudioInputPacket {
    fn drop(&mut self) {
        if !self.packet.is_null() {
            unsafe { sdk::cdecklink_audio_input_packet_release(self.packet) };
            self.packet = null_mut();
        }
    }
}

impl DecklinkAudioInputPacket {
    pub fn get_sample_frame_count(&self) -> i64 {
        unsafe { sdk::cdecklink_audio_input_packet_get_sample_frame_count(self.packet) }
    }
    pub fn get_byte_count(&self) -> usize {
        self.get_sample_frame_count() as usize * (self.channels * (self.sample_depth / 8)) as usize
    }
    pub fn get_bytes(&self) -> Result<&[u8], SdkError> {
        let mut bytes = null_mut();
        let result =
            unsafe { sdk::cdecklink_audio_input_packet_get_bytes(self.packet, &mut bytes) };

        SdkError::result_or_else(result, || unsafe {
            slice::from_raw_parts(bytes as *mut u8, self.get_byte_count())
        })
    }
    pub fn get_packet_time(&self, timescale: i64) -> Result<i64, SdkError> {
        let mut time = 0;
        let result = unsafe {
            sdk::cdecklink_audio_input_packet_get_packet_time(self.packet, &mut time, timescale)
        };
        SdkError::result_or(result, time)
    }
}
