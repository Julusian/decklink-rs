use crate::device::output::DecklinkOutputDevicePtr;
use crate::{sdk, SdkError};
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub(crate) fn wrap_audio(ptr: &Arc<DecklinkOutputDevicePtr>) -> DecklinkOutputDeviceAudio {
    DecklinkOutputDeviceAudio { ptr: ptr.clone() }
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
