use crate::{sdk, SdkError};
use num_traits::FromPrimitive;
use std::ptr::null_mut;

#[derive(EnumIter, FromPrimitive, PartialEq, Debug, Copy, Clone)]
pub enum DecklinkPixelFormat {
    Format8BitYUV = sdk::_DecklinkPixelFormat_decklinkFormat8BitYUV as isize,
    Format10BitYUV = sdk::_DecklinkPixelFormat_decklinkFormat10BitYUV as isize,
    Format8BitARGB = sdk::_DecklinkPixelFormat_decklinkFormat8BitARGB as isize,
    Format8BitBGRA = sdk::_DecklinkPixelFormat_decklinkFormat8BitBGRA as isize,
    Format10BitRGB = sdk::_DecklinkPixelFormat_decklinkFormat10BitRGB as isize,
    Format12BitRGB = sdk::_DecklinkPixelFormat_decklinkFormat12BitRGB as isize,
    Format12BitRGBLE = sdk::_DecklinkPixelFormat_decklinkFormat12BitRGBLE as isize,
    Format10BitRGBXLE = sdk::_DecklinkPixelFormat_decklinkFormat10BitRGBXLE as isize,
    Format10BitRGBX = sdk::_DecklinkPixelFormat_decklinkFormat10BitRGBX as isize,
    FormatH265 = sdk::_DecklinkPixelFormat_decklinkFormatH265 as isize,
    FormatDNxHR = sdk::_DecklinkPixelFormat_decklinkFormatDNxHR as isize,
    Format12BitRAWGRBG = sdk::_DecklinkPixelFormat_decklinkFormat12BitRAWGRBG as isize,
    Format12BitRAWJPEG = sdk::_DecklinkPixelFormat_decklinkFormat12BitRAWJPEG as isize,
}

bitflags! {
    pub struct DecklinkFrameFlags: u32 {
        const FLIP_VERTICAL = sdk::_DecklinkFrameFlags_decklinkFrameFlagFlipVertical as u32;
        const CONTAINS_HDR_METADATA = sdk::_DecklinkFrameFlags_decklinkFrameContainsHDRMetadata as u32;
        const CONTAINS_CINTEL_METADATA = sdk::_DecklinkFrameFlags_decklinkFrameContainsCintelMetadata as u32;
        const HAS_NO_INPUT_SOURCE = sdk::_DecklinkFrameFlags_decklinkFrameHasNoInputSource as u32;
    }
}

pub struct DecklinkVideoFrame {
    frame: *mut crate::sdk::cdecklink_video_frame_t,
    is_child: bool,
}

impl Drop for DecklinkVideoFrame {
    fn drop(&mut self) {
        if !self.frame.is_null() {
            if !self.is_child {
                unsafe { sdk::cdecklink_video_frame_release(self.frame) };
            }
            self.frame = null_mut();
        }
    }
}

impl DecklinkVideoFrame {
    pub fn width(&self) -> i64 {
        unsafe { sdk::cdecklink_video_frame_get_width(self.frame) }
    }
    pub fn height(&self) -> i64 {
        unsafe { sdk::cdecklink_video_frame_get_height(self.frame) }
    }
    pub fn row_bytes(&self) -> i64 {
        unsafe { sdk::cdecklink_video_frame_get_row_bytes(self.frame) }
    }
    pub fn pixel_format(&self) -> DecklinkPixelFormat {
        DecklinkPixelFormat::from_u32(unsafe {
            sdk::cdecklink_video_frame_get_pixel_format(self.frame)
        })
        .unwrap_or(DecklinkPixelFormat::Format8BitYUV)
    }
    pub fn flags(&self) -> DecklinkFrameFlags {
        DecklinkFrameFlags::from_bits_truncate(unsafe {
            sdk::cdecklink_video_frame_get_flags(self.frame)
        })
    }
    pub fn bytes(&self) {
        // TODO
        //        unsafe { sdk::cdecklink_video_frame_bytes()}
    }
}

pub struct DecklinkVideoMutableFrame {
    base: DecklinkVideoFrame,
    frame: *mut crate::sdk::cdecklink_mutable_video_frame_t,
}

impl Drop for DecklinkVideoMutableFrame {
    fn drop(&mut self) {
        if !self.frame.is_null() {
            unsafe { sdk::cdecklink_mutable_video_frame_release(self.frame) };
            self.frame = null_mut();
        }
    }
}

impl DecklinkVideoMutableFrame {
    pub fn base(&self) -> &DecklinkVideoFrame {
        &self.base
    }

    pub fn set_flags(&mut self, flags: DecklinkFrameFlags) {
        unsafe { sdk::cdecklink_mutable_video_frame_set_flags(self.frame, flags.bits()) };
    }

    pub fn set_bytes(&mut self, data: &[u8]) -> bool {
        let expected_len = (self.base.row_bytes() * self.base.height()) as usize;
        if data.len() != expected_len {
            false
        } else {
            let mut bytes = null_mut();
            unsafe {
                let r = sdk::cdecklink_video_frame_get_bytes(self.frame, &mut bytes);
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

pub unsafe fn wrap_mutable_frame(
    ptr: *mut sdk::cdecklink_mutable_video_frame_t,
) -> DecklinkVideoMutableFrame {
    DecklinkVideoMutableFrame {
        frame: ptr,
        base: DecklinkVideoFrame {
            frame: sdk::cdecklink_mutable_video_frame_to_video_frame(ptr),
            is_child: true,
        },
    }
}
pub unsafe fn wrap_frame(ptr: *mut sdk::cdecklink_video_frame_t) -> DecklinkVideoFrame {
    sdk::cdecklink_video_frame_add_ref(ptr); // TODO - all types should do this
    DecklinkVideoFrame {
        frame: ptr,
        is_child: false,
    }
}
pub unsafe fn unwrap_frame(frame: &DecklinkVideoFrame) -> *mut sdk::cdecklink_video_frame_t {
    frame.frame
}
