use crate::device::status::DecklinkStatusId;
use crate::{sdk, SdkError};
use num_traits::FromPrimitive;
use std::ptr::null_mut;
use std::sync::Arc;

#[derive(FromPrimitive, PartialEq, Debug, Copy, Clone)]
pub enum NotificationTopic {
    PreferencesChanged = sdk::_DecklinkNotifications_decklinkPreferencesChanged as isize,
    StatusChanged = sdk::_DecklinkNotifications_decklinkStatusChanged as isize,
}

pub fn wrap_notification(ptr: *mut sdk::cdecklink_status_t) -> Arc<DecklinkDeviceNotification> {
    Arc::new(DecklinkDeviceNotification { dev: ptr })
}

pub struct DecklinkDeviceNotification {
    dev: *mut sdk::cdecklink_status_t,
}

pub trait DecklinkDeviceNotificationExt {
    fn subscribe(
        &self,
        topic: NotificationTopic,
        handler: Arc<DeckLinkNotificationCallback>,
    ) -> Result<DeckLinkNotificationCallbackHandle, SdkError>;
}

impl DecklinkDeviceNotificationExt for Arc<DecklinkDeviceNotification> {
    fn subscribe(
        &self,
        topic: NotificationTopic,
        handler: Arc<DeckLinkNotificationCallback>,
    ) -> Result<DeckLinkNotificationCallbackHandle, SdkError> {
        let ptr = Box::into_raw(Box::new(DecklinkNotificationWrapper {
            handler,
            topic: topic as u32,
        }));

        let mut unsubscribe_token = null_mut(); // c++ handle, needed to call unsubscribe
        let result = unsafe {
            sdk::cdecklink_notification_subscribe(
                self.dev,
                topic as u32,
                ptr as *mut std::ffi::c_void,
                Some(notify_callback),
                &mut unsubscribe_token,
            )
        };
        SdkError::result_or_else(result, || DeckLinkNotificationCallbackHandle {
            parent: self.clone(),
            wrapper: ptr,
            unsubscribe_token,
        })
    }
}

pub struct DeckLinkNotificationCallbackHandle {
    parent: Arc<DecklinkDeviceNotification>,
    wrapper: *mut DecklinkNotificationWrapper,
    unsubscribe_token: *mut std::os::raw::c_void,
}
impl Drop for DeckLinkNotificationCallbackHandle {
    fn drop(&mut self) {
        unsafe {
            sdk::cdecklink_notification_unsubscribe(
                self.parent.dev,
                (*self.wrapper).topic,
                self.unsubscribe_token,
            );
            Box::from_raw(self.wrapper); // Reclaim the box so it gets freed
        }
    }
}

pub trait DeckLinkNotificationCallback {
    fn notify_status(&self, id: DecklinkStatusId) -> bool;
}
struct DecklinkNotificationWrapper {
    handler: Arc<DeckLinkNotificationCallback>,
    topic: u32,
}

extern "C" fn notify_callback(
    context: *mut ::std::os::raw::c_void,
    topic: sdk::DecklinkNotifications,
    param1: u64,
    _param2: u64,
) -> sdk::HRESULT {
    let wrapper: &mut DecklinkNotificationWrapper = unsafe { &mut *(context as *mut _) };

    let mut result = true;
    if topic == wrapper.topic {
        let status_id = DecklinkStatusId::from_u64(param1);
        if let Some(status_id) = status_id {
            result = wrapper.handler.notify_status(status_id)
        } else {
            // Unmapped id field. Ignore it
        }
    }

    if result {
        0 // Ok
    } else {
        1 // False
    }
}
