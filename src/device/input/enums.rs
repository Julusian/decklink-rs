use crate::sdk;

bitflags! {
    pub struct DecklinkVideoInputFlags: u32 {
        const ENABLE_FORMAT_DETECTION = sdk::_DecklinkVideoInputFlags_decklinkVideoInputEnableFormatDetection;
        const DUAL_STREAM_3D = sdk::_DecklinkVideoInputFlags_decklinkVideoInputDualStream3D;
    }
}

bitflags! {
    pub struct DecklinkVideoInputFormatChangedEvents: u32 {
        const DISPLAY_MODE = sdk::_DecklinkVideoInputFormatChangedEvents_decklinkVideoInputDisplayModeChanged;
        const FIELD_DOMINANCE = sdk::_DecklinkVideoInputFormatChangedEvents_decklinkVideoInputFieldDominanceChanged;
        const COLORSPACE = sdk::_DecklinkVideoInputFormatChangedEvents_decklinkVideoInputColorspaceChanged;
    }
}

bitflags! {
    pub struct DecklinkDetectedVideoInputFormatFlags: u32 {
        const YCBCR422 = sdk::_DecklinkDetectedVideoInputFormatFlags_decklinkDetectedVideoInputYCbCr422;
        const RGB444 = sdk::_DecklinkDetectedVideoInputFormatFlags_decklinkDetectedVideoInputRGB444;
        const DUAL_STREAM_3D = sdk::_DecklinkDetectedVideoInputFormatFlags_decklinkDetectedVideoInputDualStream3D;
    }
}
