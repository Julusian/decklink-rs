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
    /// The detected video input mode (BMDDisplayMode), available on devices which support input format detection.
    DetectedVideoInputMode = sdk::_DecklinkStatusID_decklinkStatusDetectedVideoInputMode as isize,
    DetectedVideoInputFlags = sdk::_DecklinkStatusID_decklinkStatusDetectedVideoInputFlags as isize,
    /// The current video input mode (BMDDisplayMode).
    CurrentVideoInputMode = sdk::_DecklinkStatusID_decklinkStatusCurrentVideoInputMode as isize,
    /// The current video input pixel format (BMDPixelFormat).
    CurrentVideoInputPixelFormat =
        sdk::_DecklinkStatusID_decklinkStatusCurrentVideoInputPixelFormat as isize,
    /// The current video input flags (BMDDeckLinkVideoStatusFlags)
    CurrentVideoInputFlags = sdk::_DecklinkStatusID_decklinkStatusCurrentVideoInputFlags as isize,
    /// The current video output mode (BMDDisplayMode).
    CurrentVideoOutputMode = sdk::_DecklinkStatusID_decklinkStatusCurrentVideoOutputMode as isize,
    /// The current video output flags (BMDDeckLinkVideoStatusFlags).
    CurrentVideoOutputFlags = sdk::_DecklinkStatusID_decklinkStatusCurrentVideoOutputFlags as isize,
    /// PCIe link width, x1, x4, etc.
    PCIExpressLinkWidth = sdk::_DecklinkStatusID_decklinkStatusPCIExpressLinkWidth as isize,
    /// PCIe link speed, Gen. 1, Gen. 2, etc.
    PCIExpressLinkSpeed = sdk::_DecklinkStatusID_decklinkStatusPCIExpressLinkSpeed as isize,
    /// The last video output pixel format (BMDPixelFormat).
    LastVideoOutputPixelFormat =
        sdk::_DecklinkStatusID_decklinkStatusLastVideoOutputPixelFormat as isize,
    /// The detected reference input mode (BMDDisplayMode), available on devices which support reference input format detection.
    ReferenceSignalMode = sdk::_DecklinkStatusID_decklinkStatusReferenceSignalMode as isize,
    /// The detected reference input flags (BMDDeckLinkVideoStatusFlags), available on devices which support reference input format detection.
    ReferenceSignalFlags = sdk::_DecklinkStatusID_decklinkStatusReferenceSignalFlags as isize,
    DuplexMode = sdk::_DecklinkStatusID_decklinkStatusDuplexMode as isize,
    /// The current busy state of the device. (See BMDDeviceBusyState for more information).
    Busy = sdk::_DecklinkStatusID_decklinkStatusBusy as isize,
    /// The interchangeable panel installed (BMDPanelType).
    InterchangeablePanelType =
        sdk::_DecklinkStatusID_decklinkStatusInterchangeablePanelType as isize,

    /// True if the video input signal is locked.
    VideoInputSignalLocked = sdk::_DecklinkStatusID_decklinkStatusVideoInputSignalLocked as isize,
    /// True if the reference input signal is locked.
    ReferenceSignalLocked = sdk::_DecklinkStatusID_decklinkStatusReferenceSignalLocked as isize,

    /// The received EDID of a connected HDMI sink device.
    ReceivedEDID = sdk::_DecklinkStatusID_decklinkStatusReceivedEDID as isize,
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct DecklinkVideoStatusFlags: u32 {
        /// Progressive frames are encoded as PsF.
        const PSF = sdk::_DecklinkVideoStatusFlags_decklinkVideoStatusPsF;
        /// The video signal is dual stream 3D video.
        const DUAL_STREAM_3D = sdk::_DecklinkVideoStatusFlags_decklinkVideoStatusDualStream3D;
    }
}

#[derive(FromPrimitive, PartialEq, Debug, Copy, Clone)]
pub enum DecklinkDuplexStatus {
    /// Capable of simultaneous playback and capture.
    FullDuplex = sdk::_DecklinkDuplexStatus_decklinkDuplexStatusFullDuplex as isize,
    /// Capable of playback or capture but not both simultaneously.
    HaldDuplex = sdk::_DecklinkDuplexStatus_decklinkDuplexStatusHalfDuplex as isize,
    /// Capable of playback only or capture only.
    Simplex = sdk::_DecklinkDuplexStatus_decklinkDuplexStatusSimplex as isize,
    /// Device is inactive for this profile.
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

    fn get_bool(&self, id: u32) -> Result<bool, SdkError> {
        let mut value = false;
        let result = unsafe { sdk::cdecklink_status_get_flag(self.dev, id, &mut value) };
        SdkError::result_or(result, value)
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

    /// The detected video input mode (BMDDisplayMode), available on devices which support input format detection.
    pub fn detected_video_input_mode(&self) -> Result<DecklinkDisplayModeId, SdkError> {
        into_enum(self.get_int(sdk::_DecklinkStatusID_decklinkStatusDetectedVideoInputMode))
    }
    pub fn detected_video_input_flags(&self) -> Result<DecklinkVideoStatusFlags, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusDetectedVideoInputFlags)
            .map(|v| DecklinkVideoStatusFlags::from_bits_truncate(v as u32))
    }
    /// The current video input mode (BMDDisplayMode).
    pub fn current_video_input_mode(&self) -> Result<DecklinkDisplayModeId, SdkError> {
        into_enum(self.get_int(sdk::_DecklinkStatusID_decklinkStatusCurrentVideoInputMode))
    }
    /// The current video input pixel format (BMDPixelFormat).
    pub fn current_video_input_pixel_format(&self) -> Result<DecklinkPixelFormat, SdkError> {
        into_enum(self.get_int(sdk::_DecklinkStatusID_decklinkStatusCurrentVideoInputPixelFormat))
    }
    /// The current video input flags (BMDDeckLinkVideoStatusFlags)
    pub fn current_video_input_flags(&self) -> Result<DecklinkVideoStatusFlags, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusCurrentVideoInputFlags)
            .map(|v| DecklinkVideoStatusFlags::from_bits_truncate(v as u32))
    }
    /// The current video output mode (BMDDisplayMode).
    pub fn current_video_output_mode(&self) -> Result<DecklinkDisplayModeId, SdkError> {
        into_enum(self.get_int(sdk::_DecklinkStatusID_decklinkStatusCurrentVideoOutputMode))
    }
    /// The current video output flags (BMDDeckLinkVideoStatusFlags).
    pub fn current_video_output_flags(&self) -> Result<DecklinkVideoStatusFlags, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusCurrentVideoOutputFlags)
            .map(|v| DecklinkVideoStatusFlags::from_bits_truncate(v as u32))
    }
    /// PCIe link width, x1, x4, etc.
    pub fn pci_express_link_width(&self) -> Result<u32, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusPCIExpressLinkWidth)
            .map(|v| v as u32)
    }
    /// PCIe link speed, Gen. 1, Gen. 2, etc.
    pub fn pci_express_link_speed(&self) -> Result<u32, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusPCIExpressLinkSpeed)
            .map(|v| v as u32)
    }
    /// The last video output pixel format (BMDPixelFormat).
    pub fn last_video_output_pixel_format(&self) -> Result<DecklinkPixelFormat, SdkError> {
        into_enum(self.get_int(sdk::_DecklinkStatusID_decklinkStatusLastVideoOutputPixelFormat))
    }
    /// The detected reference input mode (BMDDisplayMode), available on devices which support reference input format detection.
    pub fn reference_signal_mode(&self) -> Result<DecklinkDisplayModeId, SdkError> {
        into_enum(self.get_int(sdk::_DecklinkStatusID_decklinkStatusReferenceSignalMode))
    }
    pub fn reference_signal_flags(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusReferenceSignalFlags)
    }
    pub fn duplex_mode(&self) -> Result<DecklinkDuplexStatus, SdkError> {
        into_enum(self.get_int(sdk::_DecklinkStatusID_decklinkStatusDuplexMode))
    }
    /// The current busy state of the device. (See BMDDeviceBusyState for more information).
    pub fn busy(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusBusy)
    }
    /// The interchangeable panel installed (BMDPanelType).
    pub fn interchangeable_panel_type(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkStatusID_decklinkStatusInterchangeablePanelType)
    }

    /// True if the video input signal is locked.
    pub fn video_input_signal_locked(&self) -> Result<bool, SdkError> {
        self.get_bool(sdk::_DecklinkStatusID_decklinkStatusVideoInputSignalLocked)
    }
    /// True if the reference input signal is locked.
    pub fn reference_signal_locked(&self) -> Result<bool, SdkError> {
        self.get_bool(sdk::_DecklinkStatusID_decklinkStatusReferenceSignalLocked)
    }

    /// The received EDID of a connected HDMI sink device.
    pub fn received_edid(&self) -> Result<Vec<u8>, SdkError> {
        self.get_bytes(sdk::_DecklinkStatusID_decklinkStatusReceivedEDID)
    }
}
