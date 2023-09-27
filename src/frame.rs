use crate::{sdk, SdkError};
use aligned_vec::{AVec, ConstAlign};
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct DecklinkFrameFlags: u32 {
        const FLIP_VERTICAL = sdk::_DecklinkFrameFlags_decklinkFrameFlagFlipVertical;
        const CONTAINS_HDR_METADATA = sdk::_DecklinkFrameFlags_decklinkFrameContainsHDRMetadata;
        const CONTAINS_CINTEL_METADATA = sdk::_DecklinkFrameFlags_decklinkFrameContainsCintelMetadata;
        const HAS_NO_INPUT_SOURCE = sdk::_DecklinkFrameFlags_decklinkFrameHasNoInputSource;
    }
}

/// A frame of video
pub trait DecklinkFrameBase {
    /// Get the width of the video frame
    fn width(&self) -> usize;
    /// Get the height of the video frame
    fn height(&self) -> usize;
    /// Get the byte count per row of the video frame
    fn row_bytes(&self) -> usize;
    /// Get the pixel format of the video frame
    fn pixel_format(&self) -> DecklinkPixelFormat;
    /// Get the flags of the video frame
    fn flags(&self) -> DecklinkFrameFlags;
    /// Get the pixel data of the video frame
    fn bytes(&self) -> Result<DecklinkAlignedBytes, SdkError>;
}
pub trait DecklinkFrameBase2: DecklinkFrameBase {
    /// Get the pixel data of the video frame
    fn into_avec(self: Box<Self>) -> Result<DecklinkAlignedVec, SdkError>;
}

#[repr(align(64))]
pub struct DecklinkAlignedBytes<'a>(pub &'a [u8]);

/// Decklinks require byte arrays to be aligned to 64byte boundaries
pub type DecklinkAlignedVec = AVec<u8, ConstAlign<64>>;

/// This represents a video frame that has been received from a decklink device.
pub struct DecklinkVideoFrame {
    frame: *mut crate::sdk::cdecklink_video_frame_t,
}

impl Drop for DecklinkVideoFrame {
    fn drop(&mut self) {
        if !self.frame.is_null() {
            unsafe { sdk::cdecklink_video_frame_release(self.frame) };
            self.frame = null_mut();
        }
    }
}

impl DecklinkFrameBase for DecklinkVideoFrame {
    /// Get the width of the video frame
    fn width(&self) -> usize {
        assert!(!self.frame.is_null());

        let width = unsafe { sdk::cdecklink_video_frame_get_width(self.frame) };
        width as usize
    }
    /// Get the height of the video frame
    fn height(&self) -> usize {
        assert!(!self.frame.is_null());

        let height = unsafe { sdk::cdecklink_video_frame_get_height(self.frame) };
        height as usize
    }
    /// Get the byte count per row of the video frame
    fn row_bytes(&self) -> usize {
        assert!(!self.frame.is_null());

        let row_bytes = unsafe { sdk::cdecklink_video_frame_get_row_bytes(self.frame) };
        row_bytes as usize
    }
    /// Get the pixel format of the video frame
    fn pixel_format(&self) -> DecklinkPixelFormat {
        assert!(!self.frame.is_null());

        let format = unsafe { sdk::cdecklink_video_frame_get_pixel_format(self.frame) };

        DecklinkPixelFormat::from_u32(format).unwrap_or(DecklinkPixelFormat::Format8BitYUV)
    }
    /// Get the flags of the video frame
    fn flags(&self) -> DecklinkFrameFlags {
        assert!(!self.frame.is_null());

        let flags = unsafe { sdk::cdecklink_video_frame_get_flags(self.frame) };

        DecklinkFrameFlags::from_bits_truncate(flags)
    }

    fn bytes(&self) -> Result<DecklinkAlignedBytes, SdkError> {
        self.bytes_handle()
    }
}

impl DecklinkVideoFrame {
    /// Get the pixel data of the video frame
    pub fn bytes_to_vec(&self) -> Result<Vec<u8>, SdkError> {
        assert!(!self.frame.is_null());

        let bytes = null_mut();
        let result = unsafe { sdk::cdecklink_video_frame_get_bytes(self.frame, bytes) };
        SdkError::result(result)?;

        assert!(!bytes.is_null());

        let byte_count = self.row_bytes() * self.height();
        let mut result = vec![0; byte_count];

        unsafe { std::ptr::copy(bytes as *const u8, result.as_mut_ptr(), byte_count) };

        Ok(result)
    }

    /// Get the pixel data of the video frame
    pub fn bytes_handle(&self) -> Result<DecklinkAlignedBytes, SdkError> {
        assert!(!self.frame.is_null());

        let bytes = null_mut();
        let result = unsafe { sdk::cdecklink_video_frame_get_bytes(self.frame, bytes) };
        SdkError::result(result)?;

        assert!(!bytes.is_null());

        let byte_count = self.row_bytes() * self.height();

        let slice = unsafe { std::slice::from_raw_parts(bytes as *const u8, byte_count) };
        Ok(DecklinkAlignedBytes(slice))
    }

    // /// Get the raw pointer for the wrapped frame
    // pub(crate) unsafe fn get_cdecklink_ptr(&self) -> *mut sdk::cdecklink_video_frame_t {
    //     self.frame
    // }
    /// Wrap a raw pointer
    pub(crate) unsafe fn from(ptr: *mut sdk::cdecklink_video_frame_t) -> Self {
        sdk::cdecklink_mutable_video_frame_add_ref(ptr); // TODO - all types should do this
        Self { frame: ptr }
    }
}

pub struct DecklinkVideoMutableFrame {
    width: usize,
    height: usize,
    row_bytes: usize,
    pixel_format: DecklinkPixelFormat,
    flags: DecklinkFrameFlags,

    bytes: Option<DecklinkAlignedVec>,
}
impl DecklinkFrameBase for DecklinkVideoMutableFrame {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn row_bytes(&self) -> usize {
        self.row_bytes
    }

    fn pixel_format(&self) -> DecklinkPixelFormat {
        self.pixel_format
    }

    fn flags(&self) -> DecklinkFrameFlags {
        self.flags
    }

    fn bytes(&self) -> Result<DecklinkAlignedBytes, SdkError> {
        if let Some(bytes) = &self.bytes {
            Ok(DecklinkAlignedBytes(&bytes))
        } else {
            Err(SdkError::FALSE)
        }
    }
}
impl DecklinkFrameBase2 for DecklinkVideoMutableFrame {
    fn into_avec(self: Box<Self>) -> Result<DecklinkAlignedVec, SdkError> {
        if let Some(bytes) = self.bytes {
            Ok(bytes)
        } else {
            Err(SdkError::FALSE)
        }
    }
}
impl DecklinkVideoMutableFrame {
    pub fn create(
        width: usize,
        height: usize,
        row_bytes: usize,
        pixel_format: DecklinkPixelFormat,
        flags: DecklinkFrameFlags,
    ) -> Self {
        Self {
            width,
            height,
            row_bytes,
            pixel_format,
            flags,
            bytes: None,
        }
    }

    pub fn set_bytes(&mut self, bytes: DecklinkAlignedVec) -> Result<(), SdkError> {
        if bytes.len() < self.row_bytes * self.height {
            Err(SdkError::INVALIDARG)
        } else {
            self.bytes = Some(bytes);
            Ok(())
        }
    }

    pub fn copy_bytes(&mut self, bytes: &[u8]) -> Result<(), SdkError> {
        let byte_count = self.row_bytes * self.height;

        if bytes.len() < byte_count {
            Err(SdkError::INVALIDARG)
        } else {
            if let Some(current_bytes) = &mut self.bytes {
                if current_bytes.len() < byte_count {
                    // TODO - this may not be very performant?
                    self.bytes = Some(AVec::from_slice(64, bytes));
                } else {
                    // AVec::new(64)
                    current_bytes.copy_from_slice(bytes);
                }
            } else {
                self.bytes = Some(AVec::from_slice(64, bytes));
            }

            Ok(())
        }
    }
}
