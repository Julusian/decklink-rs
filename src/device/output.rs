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

impl Drop for DecklinkOutputDevice {
    fn drop(&mut self) {
        if !self.dev.is_null() {
            unsafe { sdk::cdecklink_destroy_device_output(self.dev) };
            self.dev = null_mut();
        }
    }
}

impl DecklinkOutputDevice {
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
            SdkError::err_or_ok(result, || ())
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
            SdkError::err_or_ok(result, || ())
        }
    }
}
