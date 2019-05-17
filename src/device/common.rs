use crate::sdk;

#[derive(FromPrimitive, PartialEq, Debug, Copy, Clone)]
pub enum DecklinkAudioSampleType {
    Int16 = sdk::_DecklinkAudioSampleType_decklinkAudioSampleType16bitInteger as isize,
    Int32 = sdk::_DecklinkAudioSampleType_decklinkAudioSampleType32bitInteger as isize,
}
