use crate::sdk;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct DecklinkVideoConnection: u32 {
        const SDI = sdk::_DecklinkVideoConnection_decklinkVideoConnectionSDI;
        const HDMI = sdk::_DecklinkVideoConnection_decklinkVideoConnectionHDMI;
        const OPTICAL_SDI = sdk::_DecklinkVideoConnection_decklinkVideoConnectionOpticalSDI;
        const COMPONENT = sdk::_DecklinkVideoConnection_decklinkVideoConnectionComponent;
        const COMPOSITE = sdk::_DecklinkVideoConnection_decklinkVideoConnectionComposite;
        const SVIDEO = sdk::_DecklinkVideoConnection_decklinkVideoConnectionSVideo;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct DecklinkAudioConnection: u32 {
        const EMBEDDED = sdk::_DecklinkAudioConnection_decklinkAudioConnectionEmbedded;
        const AES_EBU = sdk::_DecklinkAudioConnection_decklinkAudioConnectionAESEBU;
        const ANALOG = sdk::_DecklinkAudioConnection_decklinkAudioConnectionAnalog;
        const ANALOG_XLR = sdk::_DecklinkAudioConnection_decklinkAudioConnectionAnalogXLR;
        const ANALOG_RCA = sdk::_DecklinkAudioConnection_decklinkAudioConnectionAnalogRCA;
        const MICROPHONE = sdk::_DecklinkAudioConnection_decklinkAudioConnectionMicrophone;
        const HEADPHONES = sdk::_DecklinkAudioConnection_decklinkAudioConnectionHeadphones;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct DecklinkDeckControlConnection: u32 {
        const RS422_REMOTE_1 = sdk::_DecklinkDeckControlConnection_decklinkDeckControlConnectionRS422Remote1;
        const RS422_REMOTE_2 = sdk::_DecklinkDeckControlConnection_decklinkDeckControlConnectionRS422Remote2;
    }
}
