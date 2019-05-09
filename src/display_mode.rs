use crate::util::convert_string;
use crate::{sdk, SdkError};
use num_traits::FromPrimitive;
use std::ptr::{null, null_mut};

#[derive(FromPrimitive, PartialEq, Debug, Copy, Clone)]
pub enum DecklinkDisplayModeId {
    NTSC = sdk::_DecklinkDisplayMode_decklinkModeNTSC as isize,
    NTSC2398 = sdk::_DecklinkDisplayMode_decklinkModeNTSC2398 as isize,
    PAL = sdk::_DecklinkDisplayMode_decklinkModePAL as isize,
    NTSCp = sdk::_DecklinkDisplayMode_decklinkModeNTSCp as isize,
    PALp = sdk::_DecklinkDisplayMode_decklinkModePALp as isize,
    HD1080p2398 = sdk::_DecklinkDisplayMode_decklinkModeHD1080p2398 as isize,
    HD1080p24 = sdk::_DecklinkDisplayMode_decklinkModeHD1080p24 as isize,
    HD1080p25 = sdk::_DecklinkDisplayMode_decklinkModeHD1080p25 as isize,
    HD1080p2997 = sdk::_DecklinkDisplayMode_decklinkModeHD1080p2997 as isize,
    HD1080p30 = sdk::_DecklinkDisplayMode_decklinkModeHD1080p30 as isize,
    HD1080i50 = sdk::_DecklinkDisplayMode_decklinkModeHD1080i50 as isize,
    HD1080i5994 = sdk::_DecklinkDisplayMode_decklinkModeHD1080i5994 as isize,
    HD1080i6000 = sdk::_DecklinkDisplayMode_decklinkModeHD1080i6000 as isize,
    HD1080p50 = sdk::_DecklinkDisplayMode_decklinkModeHD1080p50 as isize,
    HD1080p5994 = sdk::_DecklinkDisplayMode_decklinkModeHD1080p5994 as isize,
    HD1080p6000 = sdk::_DecklinkDisplayMode_decklinkModeHD1080p6000 as isize,
    HD720p50 = sdk::_DecklinkDisplayMode_decklinkModeHD720p50 as isize,
    HD720p5994 = sdk::_DecklinkDisplayMode_decklinkModeHD720p5994 as isize,
    HD720p60 = sdk::_DecklinkDisplayMode_decklinkModeHD720p60 as isize,
    HD2k2398 = sdk::_DecklinkDisplayMode_decklinkMode2k2398 as isize,
    HD2k24 = sdk::_DecklinkDisplayMode_decklinkMode2k24 as isize,
    HD2k25 = sdk::_DecklinkDisplayMode_decklinkMode2k25 as isize,
    HD2kDCI2398 = sdk::_DecklinkDisplayMode_decklinkMode2kDCI2398 as isize,
    HD2kDCI24 = sdk::_DecklinkDisplayMode_decklinkMode2kDCI24 as isize,
    HD2kDCI25 = sdk::_DecklinkDisplayMode_decklinkMode2kDCI25 as isize,
    UHD4K2160p2398 = sdk::_DecklinkDisplayMode_decklinkMode4K2160p2398 as isize,
    UHD4K2160p24 = sdk::_DecklinkDisplayMode_decklinkMode4K2160p24 as isize,
    UHD4K2160p25 = sdk::_DecklinkDisplayMode_decklinkMode4K2160p25 as isize,
    UHD4K2160p2997 = sdk::_DecklinkDisplayMode_decklinkMode4K2160p2997 as isize,
    UHD4K2160p30 = sdk::_DecklinkDisplayMode_decklinkMode4K2160p30 as isize,
    UHD4K2160p50 = sdk::_DecklinkDisplayMode_decklinkMode4K2160p50 as isize,
    UHD4K2160p5994 = sdk::_DecklinkDisplayMode_decklinkMode4K2160p5994 as isize,
    UHD4K2160p60 = sdk::_DecklinkDisplayMode_decklinkMode4K2160p60 as isize,
    UHD4KDCI2398 = sdk::_DecklinkDisplayMode_decklinkMode4kDCI2398 as isize,
    UHD4KDCI24 = sdk::_DecklinkDisplayMode_decklinkMode4kDCI24 as isize,
    UHD4KDCI25 = sdk::_DecklinkDisplayMode_decklinkMode4kDCI25 as isize,
    CintelRAW = sdk::_DecklinkDisplayMode_decklinkModeCintelRAW as isize,
    CintelCompressedRAW = sdk::_DecklinkDisplayMode_decklinkModeCintelCompressedRAW as isize,
    Unknown = sdk::_DecklinkDisplayMode_decklinkModeUnknown as isize,
}

#[derive(FromPrimitive, PartialEq, Debug, Copy, Clone)]
pub enum DecklinkFieldDominance {
    Unknown = sdk::_DecklinkFieldDominance_decklinkUnknownFieldDominance as isize,
    LowerFieldFirst = sdk::_DecklinkFieldDominance_decklinkLowerFieldFirst as isize,
    UpperFieldFirst = sdk::_DecklinkFieldDominance_decklinkUpperFieldFirst as isize,
    ProgressiveFrame = sdk::_DecklinkFieldDominance_decklinkProgressiveFrame as isize,
    ProgressiveSegmentedFrame =
        sdk::_DecklinkFieldDominance_decklinkProgressiveSegmentedFrame as isize,
}

bitflags! {
    pub struct DecklinkDisplayModeFlag: u32 {
        const SUPPORTS_3D = sdk::_DecklinkDisplayModeFlags_decklinkDisplayModeSupports3D;
        const COLORSPACE_REC601 = sdk::_DecklinkDisplayModeFlags_decklinkDisplayModeColorspaceRec601;
        const COLORSPACE_REC709 = sdk::_DecklinkDisplayModeFlags_decklinkDisplayModeColorspaceRec709;
    }
}

pub struct DecklinkDisplayMode {
    mode: *mut sdk::cdecklink_display_mode_t,
}

impl Drop for DecklinkDisplayMode {
    fn drop(&mut self) {
        if !self.mode.is_null() {
            unsafe { sdk::cdecklink_display_mode_release(self.mode) };
            self.mode = null_mut();
        }
    }
}

impl DecklinkDisplayMode {
    pub fn name(&self) -> Option<String> {
        let mut s = null();
        unsafe { convert_string(sdk::cdecklink_display_mode_get_name(self.mode, &mut s), s) }
    }
    pub fn mode(&self) -> DecklinkDisplayModeId {
        DecklinkDisplayModeId::from_u32(unsafe {
            sdk::cdecklink_display_mode_get_display_mode(self.mode)
        })
        .unwrap_or(DecklinkDisplayModeId::Unknown)
    }
    pub fn width(&self) -> i64 {
        unsafe { sdk::cdecklink_display_mode_get_width(self.mode) }
    }
    pub fn height(&self) -> i64 {
        unsafe { sdk::cdecklink_display_mode_get_height(self.mode) }
    }
    pub fn framerate(&self) -> Option<(i64, i64)> {
        unsafe {
            let mut duration = 0;
            let mut scale = 0;
            if SdkError::is_ok(sdk::cdecklink_display_mode_get_frame_rate(
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
            sdk::cdecklink_display_mode_get_field_dominance(self.mode)
        })
        .unwrap_or(DecklinkFieldDominance::Unknown)
    }
    pub fn flags(&self) -> DecklinkDisplayModeFlag {
        DecklinkDisplayModeFlag::from_bits_truncate(unsafe {
            sdk::cdecklink_display_mode_get_flags(self.mode)
        })
    }
}

pub unsafe fn iterate_display_modes(
    it: *mut sdk::cdecklink_display_mode_iterator_t,
) -> Result<Vec<DecklinkDisplayMode>, SdkError> {
    let mut res = Vec::new();

    let mut mode = null_mut();
    loop {
        let ok2 = sdk::cdecklink_display_mode_iterator_next(it, &mut mode);
        if SdkError::is_ok(ok2) {
            res.push(DecklinkDisplayMode { mode })
        } else if SdkError::is_false(ok2) {
            break;
        } else {
            return Err(SdkError::from(ok2));
        }
    }

    Ok(res)
}

pub unsafe fn wrap_display_mode(ptr: *mut sdk::cdecklink_display_mode_t) -> DecklinkDisplayMode {
    DecklinkDisplayMode { mode: ptr }
}
