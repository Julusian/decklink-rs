use crate::device::output::{wrap_device_output, DecklinkOutputDevice};
use crate::sdk;
use crate::util::{convert_string, SdkError};
use std::ptr::{null, null_mut};

pub mod output;

pub struct DecklinkDevice {
    dev: *mut crate::sdk::cdecklink_device_t,
}

impl Drop for DecklinkDevice {
    fn drop(&mut self) {
        if !self.dev.is_null() {
            unsafe { sdk::cdecklink_device_release(self.dev) };
            self.dev = null_mut();
        }
    }
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

    pub fn output(&self) -> Option<DecklinkOutputDevice> {
        let mut output = null_mut();
        let res = unsafe { sdk::cdecklink_device_query_output(self.dev, &mut output) };
        if !SdkError::is_ok(res) || output.is_null() {
            None
        } else {
            Some(wrap_device_output(output))
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
                res.push(DecklinkDevice { dev });
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
