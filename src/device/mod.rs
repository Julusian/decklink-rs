use crate::device::attributes::{wrap_attributes, DecklinkDeviceAttributes};
use crate::device::input::{wrap_device_input, DecklinkInputDevice};
use crate::device::notification::{wrap_notification, DecklinkDeviceNotification};
use crate::device::output::{wrap_device_output, DecklinkOutputDevice};
use crate::device::status::{wrap_status, DecklinkDeviceStatus};
use crate::display_mode::{DecklinkDisplayMode, DecklinkDisplayModeId};
use crate::frame::DecklinkPixelFormat;
use crate::sdk;
use crate::util::{convert_string, SdkError};
use std::ptr::{null, null_mut};
use std::sync::{Arc, Mutex, Weak};

pub mod attributes;
pub mod common;
pub mod input;
pub mod notification;
pub mod output;
pub mod status;

unsafe impl Send for DecklinkDevice {}
unsafe impl Sync for DecklinkDevice {}
pub struct DecklinkDevice {
    dev: *mut crate::sdk::cdecklink_device_t,

    notification: Mutex<Weak<DecklinkDeviceNotification>>,
}

impl Drop for DecklinkDevice {
    fn drop(&mut self) {
        if !self.dev.is_null() {
            unsafe { sdk::cdecklink_device_release(self.dev) };
            self.dev = null_mut();
        }
    }
}

#[derive(FromPrimitive, PartialEq)]
pub enum DecklinkDisplayModeSupport {
    NotSupported = sdk::_DecklinkDisplayModeSupport_decklinkDisplayModeNotSupported as isize,
    Supported = sdk::_DecklinkDisplayModeSupport_decklinkDisplayModeSupported as isize,
    SupportedWithConversion =
        sdk::_DecklinkDisplayModeSupport_decklinkDisplayModeSupportedWithConversion as isize,
}

pub trait DecklinkDeviceDisplayModes<T> {
    fn does_support_video_mode(
        &self,
        mode: DecklinkDisplayModeId,
        pixel_format: DecklinkPixelFormat,
        flags: T,
    ) -> Result<(DecklinkDisplayModeSupport, Option<DecklinkDisplayMode>), SdkError>;

    fn display_modes(&self) -> Result<Vec<DecklinkDisplayMode>, SdkError>;
}

impl DecklinkDevice {
    pub fn model_name(&self) -> Option<String> {
        let mut s = null();
        unsafe { convert_string(sdk::cdecklink_device_get_model_name(self.dev, &mut s), s) }
    }
    pub fn display_name(&self) -> Option<String> {
        let mut s = null();
        unsafe { convert_string(sdk::cdecklink_device_get_display_name(self.dev, &mut s), s) }
    }

    pub fn get_attributes(&self) -> Result<DecklinkDeviceAttributes, SdkError> {
        let mut s = null_mut();
        let r = unsafe { sdk::cdecklink_device_query_attributes(self.dev, &mut s) };
        SdkError::result_or_else(r, || wrap_attributes(s))
    }
    pub fn get_status(&self) -> Result<DecklinkDeviceStatus, SdkError> {
        let mut s = null_mut();
        let r = unsafe { sdk::cdecklink_device_query_status(self.dev, &mut s) };
        SdkError::result_or_else(r, || wrap_status(s))
    }
    pub fn get_notification(&self) -> Result<Arc<DecklinkDeviceNotification>, SdkError> {
        if let Ok(mut locked) = self.notification.lock() {
            if let Some(val) = locked.upgrade() {
                Ok(val)
            } else {
                unsafe {
                    let mut s = null_mut();
                    let result = sdk::cdecklink_device_query_notification(self.dev, &mut s);
                    SdkError::result_or_else(result, || {
                        let notification = wrap_notification(s);
                        *locked = Arc::downgrade(&notification);
                        notification
                    })
                }
            }
        } else {
            Err(SdkError::HANDLE)
        }
    }

    pub fn output(&self) -> Option<DecklinkOutputDevice> {
        let mut output = null_mut();
        let res = unsafe { sdk::cdecklink_device_query_output(self.dev, &mut output) };
        if !SdkError::is_ok(res) || output.is_null() {
            None
        } else {
            Some(wrap_device_output(output))
        }
    }
    pub fn input(&self) -> Option<DecklinkInputDevice> {
        let mut input = null_mut();
        let res = unsafe { sdk::cdecklink_device_query_input(self.dev, &mut input) };
        if !SdkError::is_ok(res) || input.is_null() {
            None
        } else {
            Some(wrap_device_input(input))
        }
    }
}

pub fn get_devices() -> Result<Vec<DecklinkDevice>, SdkError> {
    let it = unsafe { sdk::cdecklink_create_decklink_iterator_instance() };
    if it.is_null() {
        Err(SdkError::FAIL)
    } else {
        let mut res = Vec::new();

        let mut dev = null_mut();
        loop {
            let ok = unsafe { sdk::cdecklink_iterator_next(it, &mut dev) };
            if SdkError::is_false(ok) {
                break;
            } else if SdkError::is_ok(ok) {
                res.push(DecklinkDevice {
                    dev,
                    notification: Mutex::new(Weak::new()),
                });
            } else {
                unsafe {
                    sdk::cdecklink_iterator_release(it);
                }
                return Err(SdkError::from(ok));
            }
        }

        unsafe {
            sdk::cdecklink_iterator_release(it);
        }
        Ok(res)
    }
}
