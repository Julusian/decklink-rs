use crate::connectors::{
    DecklinkAudioConnection, DecklinkDeckControlConnection, DecklinkVideoConnection,
};
use crate::sdk::DecklinkAttributeID;
use crate::util::convert_and_release_c_string;
use crate::{sdk, SdkError};
use std::ptr::{null, null_mut};

pub fn wrap_attributes(ptr: *mut sdk::cdecklink_attributes_t) -> DecklinkDeviceAttributes {
    DecklinkDeviceAttributes { dev: ptr }
}

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
    fn get_flag(&self, id: DecklinkAttributeID) -> Result<bool, SdkError> {
        let mut val = false;
        let result = unsafe { sdk::cdecklink_attributes_get_flag(self.dev, id, &mut val) };
        SdkError::result_or(result, val)
    }

    pub fn supports_internal_keying(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsInternalKeying)
    }
    pub fn supports_external_keying(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsExternalKeying)
    }
    pub fn supports_hd_keying(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsHDKeying)
    }
    pub fn supports_input_format_detection(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsInputFormatDetection)
    }
    pub fn has_reference_input(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkHasReferenceInput)
    }
    pub fn has_serial_port(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkHasSerialPort)
    }
    pub fn has_analog_video_output_gain(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkHasAnalogVideoOutputGain)
    }
    pub fn can_only_adjust_overall_video_output_gain(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkCanOnlyAdjustOverallVideoOutputGain)
    }
    pub fn has_video_input_anti_aliasing_filter(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkHasVideoInputAntiAliasingFilter)
    }
    pub fn has_bypass(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkHasBypass)
    }
    pub fn supports_clock_timing_adjustment(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsClockTimingAdjustment)
    }
    pub fn supports_full_duplex(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsFullDuplex)
    }
    pub fn supports_full_frame_reference_input_timing_offset(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsFullFrameReferenceInputTimingOffset)
    }
    pub fn supports_smpte_level_a_output(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsSMPTELevelAOutput)
    }
    pub fn supports_dual_link_sdi(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsDualLinkSDI)
    }
    pub fn supports_quad_link_sdi(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsQuadLinkSDI)
    }
    pub fn supports_idle_output(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsIdleOutput)
    }
    pub fn has_ltc_timecode_input(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkHasLTCTimecodeInput)
    }
    pub fn supports_duplex_mode_configuration(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsDuplexModeConfiguration)
    }
    pub fn supports_hdr_metadata(&self) -> Result<bool, SdkError> {
        self.get_flag(sdk::_DecklinkAttributeID_decklinkSupportsHDRMetadata)
    }

    fn get_int(&self, id: DecklinkAttributeID) -> Result<i64, SdkError> {
        let mut val = 0;
        let result = unsafe { sdk::cdecklink_attributes_get_int(self.dev, id, &mut val) };
        SdkError::result_or(result, val)
    }

    pub fn maximum_audio_channels(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkMaximumAudioChannels)
    }
    pub fn maximum_analog_audio_input_channels(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkMaximumAnalogAudioInputChannels)
    }
    pub fn maximum_analog_audio_output_channels(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkMaximumAnalogAudioOutputChannels)
    }
    pub fn number_of_sub_devices(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkNumberOfSubDevices)
    }
    pub fn sub_device_index(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkSubDeviceIndex)
    }
    pub fn persistent_id(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkPersistentID)
    }
    pub fn device_group_id(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkDeviceGroupID)
    }
    pub fn topological_id(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkTopologicalID)
    }
    pub fn video_output_connections(&self) -> Result<DecklinkVideoConnection, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkVideoOutputConnections)
            .map(|v| DecklinkVideoConnection::from_bits_truncate(v as u32))
    }
    pub fn video_input_connections(&self) -> Result<DecklinkVideoConnection, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkVideoInputConnections)
            .map(|v| DecklinkVideoConnection::from_bits_truncate(v as u32))
    }
    pub fn audio_output_connections(&self) -> Result<DecklinkAudioConnection, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkAudioOutputConnections)
            .map(|v| DecklinkAudioConnection::from_bits_truncate(v as u32))
    }
    pub fn audio_input_connections(&self) -> Result<DecklinkAudioConnection, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkAudioInputConnections)
            .map(|v| DecklinkAudioConnection::from_bits_truncate(v as u32))
    }
    pub fn video_io_support(&self) -> Result<i64, SdkError> {
        // TODO - return BMDVideoIOSupport
        self.get_int(sdk::_DecklinkAttributeID_decklinkVideoIOSupport)
    }
    pub fn deck_control_connections(&self) -> Result<DecklinkDeckControlConnection, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkVideoIOSupport)
            .map(|v| DecklinkDeckControlConnection::from_bits_truncate(v as u32))
    }
    pub fn device_interface(&self) -> Result<i64, SdkError> {
        // TODO - return BMDDeviceInterface
        self.get_int(sdk::_DecklinkAttributeID_decklinkVideoIOSupport)
    }
    pub fn audio_input_rca_channel_count(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkAudioInputRCAChannelCount)
    }
    pub fn audio_input_xlr_channel_count(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkAudioInputXLRChannelCount)
    }
    pub fn audio_output_rca_channel_count(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkAudioOutputRCAChannelCount)
    }
    pub fn audio_output_xlr_channel_count(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkAudioOutputXLRChannelCount)
    }
    pub fn paired_device_persistent_id(&self) -> Result<i64, SdkError> {
        self.get_int(sdk::_DecklinkAttributeID_decklinkPairedDevicePersistentID)
    }

    fn get_float(&self, id: DecklinkAttributeID) -> Result<f64, SdkError> {
        let mut val = 0.0;
        let result = unsafe { sdk::cdecklink_attributes_get_float(self.dev, id, &mut val) };
        SdkError::result_or(result, val)
    }

    pub fn video_input_gain_minimum(&self) -> Result<f64, SdkError> {
        self.get_float(sdk::_DecklinkAttributeID_decklinkVideoInputGainMinimum)
    }
    pub fn video_input_gain_maximum(&self) -> Result<f64, SdkError> {
        self.get_float(sdk::_DecklinkAttributeID_decklinkVideoInputGainMaximum)
    }
    pub fn video_output_gain_minimum(&self) -> Result<f64, SdkError> {
        self.get_float(sdk::_DecklinkAttributeID_decklinkVideoOutputGainMinimum)
    }
    pub fn video_output_gain_maximum(&self) -> Result<f64, SdkError> {
        self.get_float(sdk::_DecklinkAttributeID_decklinkVideoOutputGainMaximum)
    }
    pub fn microphone_input_gain_minimum(&self) -> Result<f64, SdkError> {
        self.get_float(sdk::_DecklinkAttributeID_decklinkMicrophoneInputGainMinimum)
    }
    pub fn microphone_input_gain_maximum(&self) -> Result<f64, SdkError> {
        self.get_float(sdk::_DecklinkAttributeID_decklinkMicrophoneInputGainMaximum)
    }

    fn get_string(&self, id: DecklinkAttributeID) -> Result<String, SdkError> {
        unsafe {
            let mut val = null();
            let result = sdk::cdecklink_attributes_get_string(self.dev, id, &mut val);
            SdkError::result_or_else(result, || convert_and_release_c_string(val))
        }
    }

    pub fn serial_port_device_name(&self) -> Result<String, SdkError> {
        self.get_string(sdk::_DecklinkAttributeID_decklinkSerialPortDeviceName)
    }
    pub fn vendor_name(&self) -> Result<String, SdkError> {
        self.get_string(sdk::_DecklinkAttributeID_decklinkVendorName)
    }
    pub fn display_name(&self) -> Result<String, SdkError> {
        self.get_string(sdk::_DecklinkAttributeID_decklinkDisplayName)
    }
    pub fn model_name(&self) -> Result<String, SdkError> {
        self.get_string(sdk::_DecklinkAttributeID_decklinkModelName)
    }
    pub fn device_handle(&self) -> Result<String, SdkError> {
        self.get_string(sdk::_DecklinkAttributeID_decklinkDeviceHandle)
    }
}
