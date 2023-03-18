use crate::device::output::enums::DecklinkOutputFrameCompletionResult;
use crate::device::output::DecklinkOutputDevicePtr;
use crate::frame::DecklinkVideoFrame;
use crate::{sdk, SdkError};
use num_traits::FromPrimitive;
use std::sync::{Arc, RwLock};

pub(crate) fn free_callback_wrapper(wrapper: *mut CallbackWrapper) {
    unsafe {
        drop(Box::from_raw(wrapper));
    }
}

pub fn register_callback(
    ptr: &Arc<DecklinkOutputDevicePtr>,
) -> Result<*mut CallbackWrapper, SdkError> {
    let callback_wrapper = Box::into_raw(Box::new(CallbackWrapper {
        handler: RwLock::new(None),
    }));

    let result = unsafe {
        sdk::cdecklink_output_set_scheduled_frame_completion_callback(
            ptr.dev,
            callback_wrapper as *mut std::ffi::c_void,
            Some(schedule_frame_completed_callback),
            Some(playback_stopped),
        )
    };

    match SdkError::result_or(result, callback_wrapper) {
        Err(e) => {
            free_callback_wrapper(callback_wrapper);
            Err(e)
        }
        Ok(v) => Ok(v),
    }
}

pub trait DeckLinkVideoOutputCallback {
    fn schedule_frame_completed_callback(
        &self,
        frame: Option<DecklinkVideoFrame>,
        result: DecklinkOutputFrameCompletionResult,
    ) -> bool;
    fn playback_stopped(&self) -> bool;
}

pub struct CallbackWrapper {
    pub handler: RwLock<Option<Arc<dyn DeckLinkVideoOutputCallback>>>,
}
extern "C" fn schedule_frame_completed_callback(
    context: *mut ::std::os::raw::c_void,
    frame: *mut sdk::cdecklink_video_frame_t,
    result: sdk::DecklinkOutputFrameCompletionResult,
) -> sdk::HRESULT {
    let wrapper: &mut CallbackWrapper = unsafe { &mut *(context as *mut _) };

    let mut res = true;
    if let Some(handler) = &*wrapper.handler.read().unwrap() {
        let frame_internal = if frame.is_null() {
            None
        } else {
            unsafe { Some(DecklinkVideoFrame::from(frame)) }
        };

        let result_internal = DecklinkOutputFrameCompletionResult::from_u32(result)
            .unwrap_or(DecklinkOutputFrameCompletionResult::Completed);

        res = handler.schedule_frame_completed_callback(frame_internal, result_internal);
    }

    if res {
        0 // Ok
    } else {
        1 // False
    }
}
extern "C" fn playback_stopped(context: *mut ::std::os::raw::c_void) -> sdk::HRESULT {
    let wrapper: &mut CallbackWrapper = unsafe { &mut *(context as *mut _) };

    let mut result = true;
    if let Some(handler) = &*wrapper.handler.read().unwrap() {
        result = handler.playback_stopped();
    }

    if result {
        0 // Ok
    } else {
        1 // False
    }
}
