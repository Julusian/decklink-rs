use crate::sdk::cdecklink_mutable_video_frame;
use crate::{sdk, SdkError};
use num_traits::FromPrimitive;
use std::ptr::null_mut;

#[derive(FromPrimitive, PartialEq)]
pub enum DecklinkPixelFormat {
    Format8BitYUV = sdk::_BMDPixelFormat_bmdFormat8BitYUV as isize,
    Format10BitYUV = sdk::_BMDPixelFormat_bmdFormat10BitYUV as isize,
    Format8BitARGB = sdk::_BMDPixelFormat_bmdFormat8BitARGB as isize,
    Format8BitBGRA = sdk::_BMDPixelFormat_bmdFormat8BitBGRA as isize,
    Format10BitRGB = sdk::_BMDPixelFormat_bmdFormat10BitRGB as isize,
    Format12BitRGB = sdk::_BMDPixelFormat_bmdFormat12BitRGB as isize,
    Format12BitRGBLE = sdk::_BMDPixelFormat_bmdFormat12BitRGBLE as isize,
    Format10BitRGBXLE = sdk::_BMDPixelFormat_bmdFormat10BitRGBXLE as isize,
    Format10BitRGBX = sdk::_BMDPixelFormat_bmdFormat10BitRGBX as isize,
    FormatH265 = sdk::_BMDPixelFormat_bmdFormatH265 as isize,
    FormatDNxHR = sdk::_BMDPixelFormat_bmdFormatDNxHR as isize,
    Format12BitRAWGRBG = sdk::_BMDPixelFormat_bmdFormat12BitRAWGRBG as isize,
    Format12BitRAWJPEG = sdk::_BMDPixelFormat_bmdFormat12BitRAWJPEG as isize,
}

bitflags! {
    pub struct DecklinkFrameFlags: u32 {
        const FLIP_VERTICAL = sdk::_BMDFrameFlags_bmdFrameFlagFlipVertical as u32;
        const CONTAINS_HDR_METADATA = sdk::_BMDFrameFlags_bmdFrameContainsHDRMetadata as u32;
        const CONTAINS_CINTEL_METADATA = sdk::_BMDFrameFlags_bmdFrameContainsCintelMetadata as u32;
        const HAS_NO_INPUT_SOURCE = sdk::_BMDFrameFlags_bmdFrameHasNoInputSource as u32;
    }
}

pub struct DecklinkVideoFrame {
    frame: *mut crate::sdk::cdecklink_video_frame,
    is_child: bool,
}

impl Drop for DecklinkVideoFrame {
    fn drop(&mut self) {
        if !self.frame.is_null() {
            if !self.is_child {
                unsafe { sdk::cdecklink_destroy_frame(self.frame) };
            }
            self.frame = null_mut();
        }
    }
}

impl DecklinkVideoFrame {
    pub fn width(&self) -> i64 {
        unsafe { sdk::cdecklink_video_frame_width(self.frame) }
    }
    pub fn height(&self) -> i64 {
        unsafe { sdk::cdecklink_video_frame_height(self.frame) }
    }
    pub fn row_bytes(&self) -> i64 {
        unsafe { sdk::cdecklink_video_frame_row_bytes(self.frame) }
    }
    pub fn pixel_format(&self) -> DecklinkPixelFormat {
        DecklinkPixelFormat::from_u32(unsafe {
            sdk::cdecklink_video_frame_pixel_format(self.frame)
        })
        .unwrap_or(DecklinkPixelFormat::Format8BitYUV)
    }
    pub fn flags(&self) -> DecklinkFrameFlags {
        DecklinkFrameFlags::from_bits_truncate(unsafe {
            sdk::cdecklink_video_frame_flags(self.frame)
        })
    }
    pub fn bytes(&self) {
        // TODO
        //        unsafe { sdk::cdecklink_video_frame_bytes()}
    }

    pub fn set_bytes(&self, data: &[u8]) -> bool {
        let expected_len = (self.row_bytes() * self.height()) as usize;
        if data.len() != expected_len {
            false
        } else {
            let mut bytes = null_mut();
            unsafe {
                let r = sdk::cdecklink_video_frame_bytes(self.frame, &mut bytes);
                if !SdkError::is_ok(r) {
                    // TODO - better
                    false
                } else {
                    std::ptr::copy(data.as_ptr(), bytes as *mut u8, expected_len);
                    true
                }
            }
        }
    }
}

pub struct DecklinkVideoMutableFrame {
    base: DecklinkVideoFrame,
    frame: *mut crate::sdk::cdecklink_mutable_video_frame,
}

impl Drop for DecklinkVideoMutableFrame {
    fn drop(&mut self) {
        if !self.frame.is_null() {
            unsafe { sdk::cdecklink_destroy_mutable_frame(self.frame) };
            self.frame = null_mut();
        }
    }
}

impl DecklinkVideoMutableFrame {
    pub fn base(&self) -> &DecklinkVideoFrame {
        &self.base
    }

    pub fn set_flags(&self, flags: DecklinkFrameFlags) {
        unsafe { sdk::cdecklink_video_mutable_frame_set_flags(self.frame, flags.bits()) };
    }
}

pub unsafe fn wrap_mutable_frame(
    ptr: *mut cdecklink_mutable_video_frame,
) -> DecklinkVideoMutableFrame {
    DecklinkVideoMutableFrame {
        frame: ptr,
        base: DecklinkVideoFrame {
            frame: sdk::cdecklink_video_mutable_frame_base(ptr),
            is_child: true,
        },
    }
}
pub unsafe fn unwrap_frame(frame: &DecklinkVideoFrame) -> *mut sdk::cdecklink_video_frame {
    frame.frame
}
