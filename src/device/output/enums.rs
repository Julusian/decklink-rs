use crate::sdk;

bitflags! {
    pub struct DecklinkVideoOutputFlags: u32 {
        const VANC = sdk::_DecklinkVideoOutputFlags_decklinkVideoOutputVANC;
        const VITC = sdk::_DecklinkVideoOutputFlags_decklinkVideoOutputVITC;
        const RP188 = sdk::_DecklinkVideoOutputFlags_decklinkVideoOutputRP188;
        const DUAL_STREAM_3D = sdk::_DecklinkVideoOutputFlags_decklinkVideoOutputDualStream3D;
    }
}

#[derive(FromPrimitive, PartialEq, Debug, Copy, Clone)]
pub enum DecklinkAudioOutputStreamType {
    Continuous = sdk::_DecklinkAudioOutputStreamType_decklinkAudioOutputStreamContinuous as isize,
    ContinuousDontResample =
        sdk::_DecklinkAudioOutputStreamType_decklinkAudioOutputStreamContinuousDontResample
            as isize,
}

#[derive(FromPrimitive, PartialEq, Debug, Copy, Clone)]
pub enum DecklinkOutputFrameCompletionResult {
    Completed = sdk::_DecklinkOutputFrameCompletionResult_decklinkOutputFrameCompleted as isize,
    DisplayedLate =
        sdk::_DecklinkOutputFrameCompletionResult_decklinkOutputFrameDisplayedLate as isize,
    Dropped = sdk::_DecklinkOutputFrameCompletionResult_decklinkOutputFrameDropped as isize,
    Flushed = sdk::_DecklinkOutputFrameCompletionResult_decklinkOutputFrameFlushed as isize,
}
