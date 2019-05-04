use crate::device::DecklinkOutputDevice;
use crate::display_mode::{iterate_display_modes, DecklinkDisplayMode, DecklinkDisplayModeId};
use crate::frame::{
    unwrap_frame, wrap_mutable_frame, DecklinkFrameFlags, DecklinkPixelFormat, DecklinkVideoFrame,
    DecklinkVideoMutableFrame,
};
use crate::{sdk, SdkError};
use std::ptr::null_mut;

bitflags! {
    pub struct DecklinkOutputFrameFlags: u32 {
        const VANC = sdk::_BMDVideoOutputFlags_bmdVideoOutputVANC;
        const VITC = sdk::_BMDVideoOutputFlags_bmdVideoOutputVITC;
        const RP188 = sdk::_BMDVideoOutputFlags_bmdVideoOutputRP188;
        const DUAL_STREAM_3D = sdk::_BMDVideoOutputFlags_bmdVideoOutputDualStream3D;
    }
}

#[derive(FromPrimitive)]
pub enum DecklinkAudioSampleRate {
    Rate48kHz = sdk::_BMDAudioSampleRate_bmdAudioSampleRate48kHz as isize,
}
#[derive(FromPrimitive)]
pub enum DecklinkAudioSampleType {
    Int16 = sdk::_BMDAudioSampleType_bmdAudioSampleType16bitInteger as isize,
    Int32 = sdk::_BMDAudioSampleType_bmdAudioSampleType32bitInteger as isize,
}
#[derive(FromPrimitive)]
pub enum DecklinkAudioOutputStreamType {
    Continuous = sdk::_BMDAudioOutputStreamType_bmdAudioOutputStreamContinuous as isize,
    ContinuousDontResample =
        sdk::_BMDAudioOutputStreamType_bmdAudioOutputStreamContinuousDontResample as isize,
}

impl Drop for DecklinkOutputDevice {
    fn drop(&mut self) {
        if !self.dev.is_null() {
            unsafe { sdk::cdecklink_destroy_device_output(self.dev) };
            self.dev = null_mut();
        }
    }
}

// TODO - this is currently a bag of methods, and it could do with some more sanity checking (eg allow schedule when video not enabled etc)
impl DecklinkOutputDevice {
    // TODO - does support display mode

    pub fn display_modes(&self) -> Result<Vec<DecklinkDisplayMode>, SdkError> {
        unsafe {
            let mut it = null_mut();
            let ok = sdk::cdecklink_device_output_display_mode_iterator(self.dev, &mut it);
            if SdkError::is_ok(ok) {
                let v = iterate_display_modes(it);
                sdk::cdecklink_destroy_display_mode_iterator(it);
                v
            } else {
                Err(SdkError::from(ok))
            }
        }
    }

    /* Video Output */

    pub fn enable_video_output(
        &self,
        mode: DecklinkDisplayModeId,
        flags: DecklinkOutputFrameFlags,
    ) -> Result<(), SdkError> {
        unsafe {
            let result = sdk::cdecklink_device_output_enable_video_output(
                self.dev,
                mode as u32,
                flags.bits(),
            );
            SdkError::result(result)
        }
    }
    pub fn disable_video_output(&self) -> SdkError {
        SdkError::from(unsafe { sdk::cdecklink_device_output_disable_video_output(self.dev) })
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
            let res = sdk::cdecklink_device_output_create_video_frame(
                self.dev,
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

    pub fn display_video_frame_sync(&self, frame: &DecklinkVideoFrame) -> Result<(), SdkError> {
        unsafe {
            let result = sdk::cdecklink_device_output_display_video_frame_sync(
                self.dev,
                unwrap_frame(frame),
            );
            SdkError::result(result)
        }
    }

    pub fn schedule_video_frame(
        &self,
        frame: &DecklinkVideoFrame,
        display_time: i64,
        duration: i64,
        scale: i64,
    ) -> Result<(), SdkError> {
        unsafe {
            let result = sdk::cdecklink_device_output_schedule_video_frame(
                self.dev,
                unwrap_frame(frame),
                display_time,
                duration,
                scale,
            );
            SdkError::result(result)
        }
    }

    pub fn buffered_video_frame_count(&self) -> Result<u32, SdkError> {
        unsafe {
            let mut count = 0;
            let result =
                sdk::cdecklink_device_output_buffered_video_frame_count(self.dev, &mut count);
            SdkError::result_or(result, count)
        }
    }

    /* Audio Output */

    pub fn enable_audio_output(
        &self,
        sample_rate: DecklinkAudioSampleRate,
        sample_type: DecklinkAudioSampleType,
        channels: u32,
        stream_type: DecklinkAudioOutputStreamType,
    ) -> Result<(), SdkError> {
        unsafe {
            let result = sdk::cdecklink_device_output_enable_audio_output(
                self.dev,
                sample_rate as u32,
                sample_type as u32,
                channels,
                stream_type as u32,
            );
            SdkError::result(result)
        }
    }

    pub fn disable_audio_output(&self) -> Result<(), SdkError> {
        unsafe {
            let result = sdk::cdecklink_device_output_disable_audio_output(self.dev);
            SdkError::result(result)
        }
    }

    //    pub fn write_audio_samples_sync(&self, )
    //    HRESULT cdecklink_device_output_write_audio_samples_sync(cdecklink_device_output_t *output, void *buffer,
    //    uint32_t sampleFrameCount, uint32_t *sampleFramesWritten);

    pub fn begin_audio_preroll(&self) -> Result<(), SdkError> {
        unsafe {
            let result = sdk::cdecklink_device_output_begin_audio_preroll(self.dev);
            SdkError::result(result)
        }
    }
    pub fn end_audio_preroll(&self) -> Result<(), SdkError> {
        unsafe {
            let result = sdk::cdecklink_device_output_end_audio_preroll(self.dev);
            SdkError::result(result)
        }
    }

    //    HRESULT cdecklink_device_output_schedule_audio_samples(cdecklink_device_output_t *output, void *buffer,
    //    uint32_t sampleFrameCount, int64_t streamTime,
    //    int64_t timeScale, uint32_t *sampleFramesWritten);

    pub fn buffered_audio_sample_frame_count(&self) -> Result<u32, SdkError> {
        unsafe {
            let mut count = 0;
            let result = sdk::cdecklink_device_output_buffered_audio_sample_frame_count(
                self.dev, &mut count,
            );
            SdkError::result_or(result, count)
        }
    }
    pub fn flush_buffered_audio_samples(&self) -> Result<(), SdkError> {
        unsafe {
            let result = sdk::cdecklink_device_output_flush_buffered_audio_samples(self.dev);
            SdkError::result(result)
        }
    }
}
