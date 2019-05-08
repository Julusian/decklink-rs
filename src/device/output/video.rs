use crate::device::output::video_callback::{CallbackWrapper, DeckLinkVideoOutputCallback};
use crate::device::output::DecklinkOutputDevicePtr;
use crate::frame::{unwrap_frame, DecklinkVideoFrame};
use crate::{sdk, SdkError};
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub fn wrap_video(
    ptr: &Arc<DecklinkOutputDevicePtr>,
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

pub trait DecklinkOutputDeviceVideo {
    // TODO - is this valid in sync context?
    fn buffered_video_frame_count(&self) -> Result<u32, SdkError>;
}
pub trait DecklinkOutputDeviceVideoSync: DecklinkOutputDeviceVideo {
    // TODO return type
    fn display_frame(&self, frame: &DecklinkVideoFrame) -> Result<(), SdkError>;
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

pub struct DecklinkOutputDeviceVideoImpl {
    ptr: Arc<DecklinkOutputDevicePtr>,
    pub callback_wrapper: *mut CallbackWrapper,
    pub scheduled_running: bool,
    pub scheduled_timescale: i64,
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

impl DecklinkOutputDeviceVideoSync for DecklinkOutputDeviceVideoImpl {
    fn display_frame(&self, frame: &DecklinkVideoFrame) -> Result<(), SdkError> {
        unsafe {
            let result =
                sdk::cdecklink_output_display_video_frame_sync(self.ptr.dev, unwrap_frame(frame));
            SdkError::result(result)
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
