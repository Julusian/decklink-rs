use crate::sdk;
use crate::util::{convert_string, SdkError};
use std::ptr::null_mut;

pub struct DecklinkDevice {
    dev: *mut crate::sdk::cdecklink_device,
}

impl Drop for DecklinkDevice {
    fn drop(&mut self) {
        if !self.dev.is_null() {
            unsafe { sdk::cdecklink_destroy_device(self.dev) };
            self.dev = null_mut();
        }
    }
}

impl DecklinkDevice {
    pub fn model_name(&self) -> String {
        unsafe { convert_string(sdk::cdecklink_device_model_name(self.dev)) }
    }
    pub fn display_name(&self) -> String {
        unsafe { convert_string(sdk::cdecklink_device_display_name(self.dev)) }
    }
}

pub fn get_devices() -> Result<Vec<DecklinkDevice>, SdkError> {
    let it = unsafe { sdk::cdecklink_create_iterator() };
    if it.is_null() {
        Err(SdkError::FAIL)
    } else {
        let mut res = Vec::new();

        let mut dev = null_mut();
        loop {
            let ok = unsafe { sdk::cdecklink_next_device(it, &mut dev) };
            if SdkError::is_false(ok) {
                break;
            } else if SdkError::succeeded(ok) {
                res.push(DecklinkDevice { dev });
            } else {
                unsafe {
                    sdk::cdecklink_destroy_iterator(it);
                }
                return Err(SdkError::from(ok));
            }
        }

        unsafe {
            sdk::cdecklink_destroy_iterator(it);
        }
        Ok(res)
    }
}
