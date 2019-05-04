use crate::util::convert_string;
use crate::{sdk, SdkError};
use num_traits::FromPrimitive;
use std::ptr::null_mut;
use crate::sdk::cdecklink_display_mode_iterator;

#[derive(FromPrimitive)]
pub enum DecklinkDisplayModeId {
    NTSC = sdk::_BMDDisplayMode_bmdModeNTSC as isize,
    NTSC2398 = sdk::_BMDDisplayMode_bmdModeNTSC2398 as isize,
    PAL = sdk::_BMDDisplayMode_bmdModePAL as isize,
    NTSCp = sdk::_BMDDisplayMode_bmdModeNTSCp as isize,
    PALp = sdk::_BMDDisplayMode_bmdModePALp as isize,
    HD1080p2398 = sdk::_BMDDisplayMode_bmdModeHD1080p2398 as isize,
    HD1080p24 = sdk::_BMDDisplayMode_bmdModeHD1080p24 as isize,
    HD1080p25 = sdk::_BMDDisplayMode_bmdModeHD1080p25 as isize,
    HD1080p2997 = sdk::_BMDDisplayMode_bmdModeHD1080p2997 as isize,
    HD1080p30 = sdk::_BMDDisplayMode_bmdModeHD1080p30 as isize,
    HD1080i50 = sdk::_BMDDisplayMode_bmdModeHD1080i50 as isize,
    HD1080i5994 = sdk::_BMDDisplayMode_bmdModeHD1080i5994 as isize,
    HD1080i6000 = sdk::_BMDDisplayMode_bmdModeHD1080i6000 as isize,
    HD1080p50 = sdk::_BMDDisplayMode_bmdModeHD1080p50 as isize,
    HD1080p5994 = sdk::_BMDDisplayMode_bmdModeHD1080p5994 as isize,
    HD1080p6000 = sdk::_BMDDisplayMode_bmdModeHD1080p6000 as isize,
    HD720p50 = sdk::_BMDDisplayMode_bmdModeHD720p50 as isize,
    HD720p5994 = sdk::_BMDDisplayMode_bmdModeHD720p5994 as isize,
    HD720p60 = sdk::_BMDDisplayMode_bmdModeHD720p60 as isize,
    HD2k2398 = sdk::_BMDDisplayMode_bmdMode2k2398 as isize,
    HD2k24 = sdk::_BMDDisplayMode_bmdMode2k24 as isize,
    HD2k25 = sdk::_BMDDisplayMode_bmdMode2k25 as isize,
    HD2kDCI2398 = sdk::_BMDDisplayMode_bmdMode2kDCI2398 as isize,
    HD2kDCI24 = sdk::_BMDDisplayMode_bmdMode2kDCI24 as isize,
    HD2kDCI25 = sdk::_BMDDisplayMode_bmdMode2kDCI25 as isize,
    UHD4K2160p2398 = sdk::_BMDDisplayMode_bmdMode4K2160p2398 as isize,
    UHD4K2160p24 = sdk::_BMDDisplayMode_bmdMode4K2160p24 as isize,
    UHD4K2160p25 = sdk::_BMDDisplayMode_bmdMode4K2160p25 as isize,
    UHD4K2160p2997 = sdk::_BMDDisplayMode_bmdMode4K2160p2997 as isize,
    UHD4K2160p30 = sdk::_BMDDisplayMode_bmdMode4K2160p30 as isize,
    UHD4K2160p50 = sdk::_BMDDisplayMode_bmdMode4K2160p50 as isize,
    UHD4K2160p5994 = sdk::_BMDDisplayMode_bmdMode4K2160p5994 as isize,
    UHD4K2160p60 = sdk::_BMDDisplayMode_bmdMode4K2160p60 as isize,
    UHD4KDCI2398 = sdk::_BMDDisplayMode_bmdMode4kDCI2398 as isize,
    UHD4KDCI24 = sdk::_BMDDisplayMode_bmdMode4kDCI24 as isize,
    UHD4KDCI25 = sdk::_BMDDisplayMode_bmdMode4kDCI25 as isize,
    CintelRAW = sdk::_BMDDisplayMode_bmdModeCintelRAW as isize,
    CintelCompressedRAW = sdk::_BMDDisplayMode_bmdModeCintelCompressedRAW as isize,
    Unknown = sdk::_BMDDisplayMode_bmdModeUnknown as isize,
}

#[derive(FromPrimitive)]
pub enum DecklinkFieldDominance {
    Unknown = sdk::_BMDFieldDominance_bmdUnknownFieldDominance as isize,
    LowerFieldFirst = sdk::_BMDFieldDominance_bmdLowerFieldFirst as isize,
    UpperFieldFirst = sdk::_BMDFieldDominance_bmdUpperFieldFirst as isize,
    ProgressiveFrame = sdk::_BMDFieldDominance_bmdProgressiveFrame as isize,
    ProgressiveSegmentedFrame = sdk::_BMDFieldDominance_bmdProgressiveSegmentedFrame as isize,
}

bitflags! {
    pub struct DecklinkDisplayModeFlag: u32 {
        const SUPPORTS_3D = sdk::_BMDDisplayModeFlags_bmdDisplayModeSupports3D;
        const COLORSPACE_REC601 = sdk::_BMDDisplayModeFlags_bmdDisplayModeColorspaceRec601;
        const COLORSPACE_REC709 = sdk::_BMDDisplayModeFlags_bmdDisplayModeColorspaceRec709;
    }
}
//#[derive(FromPrimitive)]
//pub enum DecklinkDisplayModeFlag {
//    Supports3D = sdk::_BMDDisplayModeFlags_bmdDisplayModeSupports3D as isize,
//    ColorspaceRec601 = sdk::_BMDDisplayModeFlags_bmdDisplayModeColorspaceRec601 as isize,
//    ColorspaceRec709 = sdk::_BMDDisplayModeFlags_bmdDisplayModeColorspaceRec709 as isize,
//}

pub struct DecklinkDisplayMode {
    mode: *mut crate::sdk::cdecklink_display_mode,
}

impl Drop for DecklinkDisplayMode {
    fn drop(&mut self) {
        if !self.mode.is_null() {
            // TODO
            // unsafe { sdk::cdecklink_destroy_device(self.dev) };
            self.mode = null_mut();
        }
    }
}

impl DecklinkDisplayMode {
    pub fn name(&self) -> String {
        unsafe { convert_string(sdk::cdecklink_display_mode_name(self.mode)) }
    }
    pub fn mode(&self) -> DecklinkDisplayModeId {
        DecklinkDisplayModeId::from_u32(unsafe { sdk::cdecklink_display_mode_mode(self.mode) })
            .unwrap_or(DecklinkDisplayModeId::Unknown)
    }
    pub fn width(&self) -> i64 {
        unsafe { sdk::cdecklink_display_mode_width(self.mode) }
    }
    pub fn height(&self) -> i64 {
        unsafe { sdk::cdecklink_display_mode_height(self.mode) }
    }
    pub fn framerate(&self) -> Option<(i64, i64)> {
        unsafe {
            let mut duration = 0;
            let mut scale = 0;
            if SdkError::succeeded(sdk::cdecklink_display_mode_framerate(
                self.mode,
                &mut duration,
                &mut scale,
            )) {
                Some((duration, scale))
            } else {
                None
            }
        }
    }
    pub fn field_dominance(&self) -> DecklinkFieldDominance {
        DecklinkFieldDominance::from_u32(unsafe {
            sdk::cdecklink_display_mode_field_dominance(self.mode)
        })
        .unwrap_or(DecklinkFieldDominance::Unknown)
    }
    pub fn flags(&self) -> DecklinkDisplayModeFlag {
        DecklinkDisplayModeFlag::from_bits_truncate(unsafe {
            sdk::cdecklink_display_mode_flags(self.mode)
        })
    }
}

pub unsafe fn iterate_display_modes(it: *mut cdecklink_display_mode_iterator) -> Result<Vec<DecklinkDisplayMode>, SdkError> {
    let mut res = Vec::new();

    let mut mode = null_mut();
    loop {
        let ok2 = sdk::cdecklink_next_display_mode(it, &mut mode);
        if SdkError::is_ok(ok2) {
            res.push(DecklinkDisplayMode{
                mode
            })
        } else if SdkError::is_false(ok2) {
            break
        } else {
            return Err(SdkError::from(ok2))
        }
    }

    Ok(res)
}
