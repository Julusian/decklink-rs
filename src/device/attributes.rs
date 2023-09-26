use crate::connectors::{
    DecklinkAudioConnection, DecklinkDeckControlConnection, DecklinkVideoConnection,
};
use crate::sdk::DecklinkAttributeID;
use crate::util::{convert_and_release_c_string, convert_c_string};
use crate::{sdk, SdkError};
use std::ptr::{null, null_mut};

pub struct DecklinkDeviceAttributes {
    dev: *mut sdk::cdecklink_attributes_t,
}

impl Drop for DecklinkDeviceAttributes {
    fn drop(&mut self) {
        if !self.dev.is_null() {
            unsafe { sdk::cdecklink_attributes_release(self.dev) };
            self.dev = null_mut();
        }
    }
}

impl DecklinkDeviceAttributes {
    pub(crate) fn from(ptr: *mut sdk::cdecklink_attributes_t) -> DecklinkDeviceAttributes {
        DecklinkDeviceAttributes { dev: ptr }
    }

    fn get_flag(&self, id: DecklinkAttributeID) -> Result<bool, SdkError> {
        let mut val = false;
        let result = unsafe { sdk::cdecklink_attributes_get_flag(self.dev, id, &mut val) };
        SdkError::result_or(result, val)
    }

    fn get_int(&self, id: DecklinkAttributeID) -> Result<i64, SdkError> {
        let mut val = 0;
        let result = unsafe { sdk::cdecklink_attributes_get_int(self.dev, id, &mut val) };
        SdkError::result_or(result, val)
    }

    fn get_float(&self, id: DecklinkAttributeID) -> Result<f64, SdkError> {
        let mut val = 0.0;
        let result = unsafe { sdk::cdecklink_attributes_get_float(self.dev, id, &mut val) };
        SdkError::result_or(result, val)
    }

    fn get_string_pointer(&self, id: DecklinkAttributeID) -> Result<String, SdkError> {
        unsafe {
            let mut val = null();
            let result = sdk::cdecklink_attributes_get_string(self.dev, id, &mut val);
            SdkError::result_or_else(result, || convert_and_release_c_string(val))
        }
    }

    fn get_string_from_reference(&self, id: DecklinkAttributeID) -> Result<String, SdkError> {
        unsafe {
            let mut val = null();
            let result = sdk::cdecklink_attributes_get_string(self.dev, id, &mut val);
            SdkError::result_or_else(result, || convert_c_string(val))
        }
    }

    /// True if internal keying is supported on this device.
    pub fn supports_internal_keying(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsInternalKeying)
    }
    /// True if external keying is supported on this device.
    pub fn supports_external_keying(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsExternalKeying)
    }
    pub fn supports_hd_keying(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsHDKeying)
    }
    /// True if input format detection is supported on this device.
    pub fn supports_input_format_detection(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsInputFormatDetection)
    }
    /// True if the DeckLink device has a genlock reference source input connector.
    pub fn has_reference_input(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkHasReferenceInput)
    }
    // True if device has a serial port.
    pub fn has_serial_port(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkHasSerialPort)
    }
    // True if analog video output gain adjustment is supported on this device.
    pub fn has_analog_video_output_gain(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkHasAnalogVideoOutputGain)
    }
    /// True if only the overall video output gain can be adjusted.
    /// In this case, only the luma gain can be accessed with the IDeckLinkConfiguration interface,
    /// and it controls all three gains (luma, chroma blue and chroma red).
    pub fn can_only_adjust_overall_video_output_gain(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkCanOnlyAdjustOverallVideoOutputGain)
    }
    /// True if there is an antialising filter on the analog video input of this device.
    pub fn has_video_input_anti_aliasing_filter(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkHasVideoInputAntiAliasingFilter)
    }
    /// True if this device has loop-through bypass function.
    pub fn has_bypass(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkHasBypass)
    }
    /// True if this device supports clock timing adjustment.
    /// (see bmdDeckLinkConfigClockTimingAdjustment).
    pub fn supports_clock_timing_adjustment(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsClockTimingAdjustment)
    }
    pub fn supports_full_duplex(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsFullDuplex)
    }
    /// True if the DeckLink device supports genlock offset adjustment wider than +/511 pixels
    /// (see bmdDeckLinkConfigReferenceInputTimingOffset for more information).
    pub fn supports_full_frame_reference_input_timing_offset(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsFullFrameReferenceInputTimingOffset)
    }
    /// True if SMPTE Level A output is supported on this device.
    pub fn supports_smpte_level_a_output(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsSMPTELevelAOutput)
    }
    /// True if SDI dual-link is supported on this device.
    pub fn supports_dual_link_sdi(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsDualLinkSDI)
    }
    /// True if SDI quad-link is supported on this device.
    pub fn supports_quad_link_sdi(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsQuadLinkSDI)
    }
    /// True if this device supports idle output.
    /// (see BMDIdleVideoOutputOperation for idle output options).
    pub fn supports_idle_output(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsIdleOutput)
    }
    /// True if this device has a dedicated LTC input.
    pub fn has_ltc_timecode_input(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkHasLTCTimecodeInput)
    }
    pub fn supports_duplex_mode_configuration(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsDuplexModeConfiguration)
    }
    /// True if the device supports transport of HDR metadata.
    pub fn supports_hdr_metadata(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsHDRMetadata)
    }

    /// The maximum number of embedded audio channels on digital connections supported by this device.
    pub fn maximum_audio_channels(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkMaximumAudioChannels)
    }
    /// The maximum number of input analog audio channels supported by this device.
    pub fn maximum_analog_audio_input_channels(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkMaximumAnalogAudioInputChannels)
    }
    /// The maximum number of output analog audio channels supported by this device.
    pub fn maximum_analog_audio_output_channels(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkMaximumAnalogAudioOutputChannels)
    }
    /// Some DeckLink hardware devices contain multiple independent sub-devices.
    /// This attribute will be equal to one for most devices, or two or more on a card with multiple sub-devices (eg DeckLink Duo).
    pub fn number_of_sub_devices(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkNumberOfSubDevices)
    }
    /// Some DeckLink hardware devices contain multiple independent sub-devices.
    /// This attribute indicates the index of the sub-device, starting from zero
    pub fn sub_device_index(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkSubDeviceIndex)
    }
    /// A device specific 32 bit unique identifier.
    pub fn persistent_id(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkPersistentID)
    }
    /// A 32 bit identifier used to group sub-devices belonging to the same DeckLink hardware device.
    /// Supported if the sub-device supports BMDDeckLinkPersistentID
    pub fn device_group_id(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkDeviceGroupID)
    }
    /// An identifier for DeckLink devices. This feature is supported on a given device if S_OK is returned.
    /// The ID will persist across reboots assuming that devices are not disconnected or moved to a different slot.
    pub fn topological_id(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkTopologicalID)
    }
    /// The video output connections supported by the hardware
    /// (see BMDVideoConnection for more details).
    /// Multiple video output connections can be active simultaneously.
    pub fn video_output_connections(&self) -> Result<DecklinkVideoConnection, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkVideoOutputConnections)
            .map(|v| DecklinkVideoConnection::from_bits_truncate(v as u32))
    }
    /// The video input connections supported by the hardware
    /// (see BMDVideoConnection for more details).
    pub fn video_input_connections(&self) -> Result<DecklinkVideoConnection, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkVideoInputConnections)
            .map(|v| DecklinkVideoConnection::from_bits_truncate(v as u32))
    }
    /// The audio output connections supported by the hardware
    /// (see BMDAudioConnection for more details).
    /// Multiple audio output connections can be active simultaneously.
    /// Devices with one or more types of analog connection will have the bmdAudioConnectionAnalog flag set.
    /// Devices with individually selectable XLR/RCA connectors will additionally have the bmdAudioConnectionAnalogXLR and bmdAudioConnectionAnalogRCA flags set.
    pub fn audio_output_connections(&self) -> Result<DecklinkAudioConnection, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkAudioOutputConnections)
            .map(|v| DecklinkAudioConnection::from_bits_truncate(v as u32))
    }
    /// The audio input connections supported by the hardware
    /// (see BMDAudioConnection for more details).
    pub fn audio_input_connections(&self) -> Result<DecklinkAudioConnection, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkAudioInputConnections)
            .map(|v| DecklinkAudioConnection::from_bits_truncate(v as u32))
    }
    /// The capture and/or playback capability of the device.
    /// (See BMDVideoIOSupport for more information)
    pub fn video_io_support(&self) -> Result<i64, SdkError> {
        // TODO - return BMDVideoIOSupport
        self.get_int(sdk::_DecklinkAttributeID_decklinkVideoIOSupport)
    }
    /// The deck control connections supported by the hardware
    /// (see BMDDeckControlConnection for more information).
    pub fn deck_control_connections(&self) -> Result<DecklinkDeckControlConnection, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkDeckControlConnections)
            .map(|v| DecklinkDeckControlConnection::from_bits_truncate(v as u32))
    }
    /// The active device interface
    /// (see BMDDeviceInterface for more information)
    pub fn device_interface(&self) -> Result<i64, SdkError> {
        // TODO - return BMDDeviceInterface
        self.get_int(sdk::_DecklinkAttributeID_decklinkDeviceInterface)
    }
    /// Number of input audio RCA channels supported by this device.
    pub fn audio_input_rca_channel_count(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkAudioInputRCAChannelCount)
    }
    /// Number of input audio XLR channels supported by this device
    pub fn audio_input_xlr_channel_count(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkAudioInputXLRChannelCount)
    }
    /// Number of output audio RCA channels supported by this device.
    pub fn audio_output_rca_channel_count(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkAudioOutputRCAChannelCount)
    }
    /// Number of output audio XLR channels supported by this device
    pub fn audio_output_xlr_channel_count(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkAudioOutputXLRChannelCount)
    }
    pub fn paired_device_persistent_id(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkPairedDevicePersistentID)
    }

    /// The minimum video input gain in dB for this device.
    pub fn video_input_gain_minimum(&self) -> Result<f64, SdkError> {
        self.get_float(sdk::_DecklinkAttributeID_decklinkVideoInputGainMinimum)
    }
    /// The maximum video input gain in dB for this device.
    pub fn video_input_gain_maximum(&self) -> Result<f64, SdkError> {
        self.get_float(sdk::_DecklinkAttributeID_decklinkVideoInputGainMaximum)
    }
    /// The minimum video output gain in dB for this device.
    pub fn video_output_gain_minimum(&self) -> Result<f64, SdkError> {
        self.get_float(sdk::_DecklinkAttributeID_decklinkVideoOutputGainMinimum)
    }
    /// The maximum video output gain in dB for this device.
    pub fn video_output_gain_maximum(&self) -> Result<f64, SdkError> {
        self.get_float(sdk::_DecklinkAttributeID_decklinkVideoOutputGainMaximum)
    }
    /// The minimum microphone input gain in dB for this device.
    pub fn microphone_input_gain_minimum(&self) -> Result<f64, SdkError> {
        self.get_float(sdk::_DecklinkAttributeID_decklinkMicrophoneInputGainMinimum)
    }
    /// The maximum microphone input gain in dB for this device.
    pub fn microphone_input_gain_maximum(&self) -> Result<f64, SdkError> {
        self.get_float(sdk::_DecklinkAttributeID_decklinkMicrophoneInputGainMaximum)
    }

    /// The operating system name of the RS422 serial port on this device.
    pub fn serial_port_device_name(&self) -> Result<String, SdkError> {
        self.get_string_pointer(sdk::_DecklinkAttributeID_decklinkSerialPortDeviceName)
    }
    /// Hardware vendor name.
    pub fn vendor_name(&self) -> Result<String, SdkError> {
        self.get_string_from_reference(sdk::_DecklinkAttributeID_decklinkVendorName)
    }
    /// The device’s display name.
    /// See IDeckLink::GetDisplayName.
    pub fn display_name(&self) -> Result<String, SdkError> {
        self.get_string_pointer(sdk::_DecklinkAttributeID_decklinkDisplayName)
    }
    /// Hardware Model Name.
    /// See IDeckLink::GetModelName.
    pub fn model_name(&self) -> Result<String, SdkError> {
        self.get_string_pointer(sdk::_DecklinkAttributeID_decklinkModelName)
    }
    /// String representing an unique identifier for the device.
    /// The format of the string is “RevisionID:PersistentID:TopologicalID”.
    pub fn device_handle(&self) -> Result<String, SdkError> {
        self.get_string_pointer(sdk::_DecklinkAttributeID_decklinkDeviceHandle)
    }
}
