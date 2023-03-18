use crate::display_mode::DecklinkDisplayModeId;
use crate::frame::DecklinkPixelFormat;
use crate::{sdk, SdkError};
use num_traits::FromPrimitive;
use std::os::raw::c_void;
use std::ptr::null_mut;

pub struct DecklinkDeviceStatus {
    dev: *mut sdk::cdecklink_status_t,
}

#[derive(FromPrimitive, PartialEq, Debug, Copy, Clone)]
pub enum DecklinkStatusId {
    DetectedVideoInputMode = sdk::_DecklinkStatusID_decklinkStatusDetectedVideoInputMode as isize,
    DetectedVideoInputFlags = sdk::_DecklinkStatusID_decklinkStatusDetectedVideoInputFlags as isize,
    CurrentVideoInputMode = sdk::_DecklinkStatusID_decklinkStatusCurrentVideoInputMode as isize,
    CurrentVideoInputPixelFormat =
        sdk::_DecklinkStatusID_decklinkStatusCurrentVideoInputPixelFormat as isize,
    CurrentVideoInputFlags = sdk::_DecklinkStatusID_decklinkStatusCurrentVideoInputFlags as isize,
    CurrentVideoOutputMode = sdk::_DecklinkStatusID_decklinkStatusCurrentVideoOutputMode as isize,
    CurrentVideoOutputFlags = sdk::_DecklinkStatusID_decklinkStatusCurrentVideoOutputFlags as isize,
    PCIExpressLinkWidth = sdk::_DecklinkStatusID_decklinkStatusPCIExpressLinkWidth as isize,
    PCIExpressLinkSpeed = sdk::_DecklinkStatusID_decklinkStatusPCIExpressLinkSpeed as isize,
    LastVideoOutputPixelFormat =
        sdk::_DecklinkStatusID_decklinkStatusLastVideoOutputPixelFormat as isize,
    ReferenceSignalMode = sdk::_DecklinkStatusID_decklinkStatusReferenceSignalMode as isize,
    ReferenceSignalFlags = sdk::_DecklinkStatusID_decklinkStatusReferenceSignalFlags as isize,
    DuplexMode = sdk::_DecklinkStatusID_decklinkStatusDuplexMode as isize,
    Busy = sdk::_DecklinkStatusID_decklinkStatusBusy as isize,
    InterchangeablePanelType =
        sdk::_DecklinkStatusID_decklinkStatusInterchangeablePanelType as isize,

    VideoInputSignalLocked = sdk::_DecklinkStatusID_decklinkStatusVideoInputSignalLocked as isize,
    ReferenceSignalLocked = sdk::_DecklinkStatusID_decklinkStatusReferenceSignalLocked as isize,

    ReceivedEDID = sdk::_DecklinkStatusID_decklinkStatusReceivedEDID as isize,
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct DecklinkVideoStatusFlags: u32 {
        const PSF = sdk::_DecklinkVideoStatusFlags_decklinkVideoStatusPsF;
        const DUAL_STREAM_3D = sdk::_DecklinkVideoStatusFlags_decklinkVideoStatusDualStream3D;
    }
}

#[derive(FromPrimitive, PartialEq, Debug, Copy, Clone)]
pub enum DecklinkDuplexStatus {
    FullDuplex = sdk::_DecklinkDuplexStatus_decklinkDuplexStatusFullDuplex as isize,
    HaldDuplex = sdk::_DecklinkDuplexStatus_decklinkDuplexStatusHalfDuplex as isize,
    Simplex = sdk::_DecklinkDuplexStatus_decklinkDuplexStatusSimplex as isize,
    Inactive = sdk::_DecklinkDuplexStatus_decklinkDuplexStatusInactive as isize,
}

impl Drop for DecklinkDeviceStatus {
    fn drop(&mut self) {
        if !self.dev.is_null() {
            unsafe { sdk::cdecklink_status_release(self.dev) };
            self.dev = null_mut();
        }
    }
}

fn into_enum<T>(res: Result<i64, SdkError>) -> Result<T, SdkError>
where
    T: FromPrimitive,
{
    match res {
        Err(e) => Err(e),
        Ok(v) => T::from_i64(v).ok_or(SdkError::FALSE),
    }
}

impl DecklinkDeviceStatus {
    pub(crate) fn from(ptr: *mut sdk::cdecklink_status_t) -> DecklinkDeviceStatus {
        DecklinkDeviceStatus { dev: ptr }
    }

    // TODO - do separate like attributes
    fn get_int(&self, id: u32) -> Result<i64, SdkError> {
        let mut value = 0;
        let result = unsafe { sdk::cdecklink_status_get_int(self.dev, id, &mut value) };
        SdkError::result_or(result, value)
    }

    pub fn detected_video_input_mode(&self) -> Result<DecklinkDisplayModeId, SdkError> {
        into_enum(self.get_int(sdk::_DecklinkStatusID_decklinkStatusDetectedVideoInputMode))
    }
    pub fn detected_video_input_flags(&self) -> Result<DecklinkVideoStatusFlags, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusDetectedVideoInputMode)
            .map(|v| DecklinkVideoStatusFlags::from_bits_truncate(v as u32))
    }
    pub fn current_video_input_mode(&self) -> Result<DecklinkDisplayModeId, SdkError> {
        into_enum(self.get_int(sdk::_DecklinkStatusID_decklinkStatusCurrentVideoInputMode))
    }
    pub fn current_video_input_pixel_format(&self) -> Result<DecklinkPixelFormat, SdkError> {
        into_enum(self.get_int(sdk::_DecklinkStatusID_decklinkStatusCurrentVideoInputPixelFormat))
    }
    pub fn current_video_input_flags(&self) -> Result<DecklinkVideoStatusFlags, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusCurrentVideoInputFlags)
            .map(|v| DecklinkVideoStatusFlags::from_bits_truncate(v as u32))
    }
    pub fn current_video_output_mode(&self) -> Result<DecklinkDisplayModeId, SdkError> {
        into_enum(self.get_int(sdk::_DecklinkStatusID_decklinkStatusCurrentVideoOutputMode))
    }
    pub fn current_video_output_flags(&self) -> Result<DecklinkVideoStatusFlags, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusCurrentVideoOutputFlags)
            .map(|v| DecklinkVideoStatusFlags::from_bits_truncate(v as u32))
    }
    pub fn pci_express_link_width(&self) -> Result<u32, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusPCIExpressLinkWidth)
            .map(|v| v as u32)
    }
    pub fn pci_express_link_speed(&self) -> Result<u32, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusPCIExpressLinkSpeed)
            .map(|v| v as u32)
    }
    pub fn last_video_output_pixel_format(&self) -> Result<DecklinkPixelFormat, SdkError> {
        into_enum(self.get_int(sdk::_DecklinkStatusID_decklinkStatusLastVideoOutputPixelFormat))
    }
    pub fn reference_signal_mode(&self) -> Result<DecklinkDisplayModeId, SdkError> {
        into_enum(self.get_int(sdk::_DecklinkStatusID_decklinkStatusReferenceSignalMode))
    }
    pub fn reference_signal_flags(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusReferenceSignalFlags)
    }
    pub fn duplex_mode(&self) -> Result<DecklinkDuplexStatus, SdkError> {
        into_enum(self.get_int(sdk::_DecklinkStatusID_decklinkStatusDuplexMode))
    }
    pub fn busy(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusBusy)
    }
    pub fn interchangeable_panel_type(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusInterchangeablePanelType)
    }

    fn get_bool(&self, id: u32) -> Result<bool, SdkError> {
        let mut value = false;
        let result = unsafe { sdk::cdecklink_status_get_flag(self.dev, id, &mut value) };
        SdkError::result_or(result, value)
    }

    pub fn video_input_signal_locked(&self) -> Result<bool, SdkError> {
        self.get_bool(sdk::_DecklinkStatusID_decklinkStatusVideoInputSignalLocked)
    }
    pub fn reference_signal_locked(&self) -> Result<bool, SdkError> {
        self.get_bool(sdk::_DecklinkStatusID_decklinkStatusReferenceSignalLocked)
    }

    fn get_bytes(&self, id: u32) -> Result<Vec<u8>, SdkError> {
        let mut byte_count = 0;
        let result =
            unsafe { sdk::cdecklink_status_get_bytes(self.dev, id, null_mut(), &mut byte_count) };
        if SdkError::is_ok(result) {
            let mut bytes = vec![0; byte_count as usize];
            let result = unsafe {
                sdk::cdecklink_status_get_bytes(
                    self.dev,
                    id,
                    bytes.as_mut_ptr() as *mut c_void,
                    &mut byte_count,
                )
            };
            SdkError::result_or(result, bytes)
        } else {
            Err(SdkError::from(result))
        }
    }

    pub fn received_edid(&self) -> Result<Vec<u8>, SdkError> {
        self.get_bytes(sdk::_DecklinkStatusID_decklinkStatusReceivedEDID)
    }
}
