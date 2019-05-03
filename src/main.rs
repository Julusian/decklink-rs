use std::ffi::CStr;
use std::ptr::null_mut;
use crate::sdk::cdecklink_device;

#[allow(
non_snake_case,
non_camel_case_types,
non_upper_case_globals,
dead_code,
clippy::all
)]
#[link(name = "decklink_c", kind = "static")]
mod sdk;

// TODO - refactor the error type to abstract away weird errors?
#[derive(Debug)]
#[allow(overflowing_literals)]
pub enum SdkError {
    FALSE = 0x00000001,
    UNEXPECTED = 0x8000FFFF,
    NOTIMPL = 0x80000001,
    OUTOFMEMORY = 0x80000002,
    INVALIDARG = 0x80000003,
    NOINTERFACE = 0x80000004,
    POINTER = 0x80000005,
    HANDLE = 0x80000006,
    ABORT = 0x80000007,
    FAIL = 0x80000008,
    ACCESSDENIED = 0x80000009,
}

impl SdkError {
    #[allow(overflowing_literals)]
    fn from(value: i32) -> SdkError {
        match value {
            0x8000FFFF => SdkError::UNEXPECTED,
            0x80000001 => SdkError::NOTIMPL,
            0x80000002 => SdkError::OUTOFMEMORY,
            0x80000003 => SdkError::INVALIDARG,
            0x80000004 => SdkError::NOINTERFACE,
            0x80000005 => SdkError::POINTER,
            0x80000006 => SdkError::HANDLE,
            0x80000007 => SdkError::ABORT,
            0x80000008 => SdkError::FAIL,
            0x80000009 => SdkError::ACCESSDENIED,
            _ => SdkError::FALSE,
        }
    }
    fn is_false(value: i32) -> bool {
        value == (SdkError::FALSE as i32)
    }
    fn succeeded(r: i32) -> bool {
        return r >= 0;
    }
}

pub struct DecklinkDevice {
    dev: *mut cdecklink_device
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
        let c_str = unsafe { CStr::from_ptr(sdk::cdecklink_device_model_name(self.dev)) };
        c_str.to_str().unwrap_or_default().to_string()
    }
    pub fn display_name(&self) -> String {
        let c_str = unsafe { CStr::from_ptr(sdk::cdecklink_device_display_name(self.dev)) };
        c_str.to_str().unwrap_or_default().to_string()
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
                unsafe { sdk::cdecklink_destroy_iterator(it); }
                return Err(SdkError::from(ok));
            }
        }

        unsafe { sdk::cdecklink_destroy_iterator(it); }
        Ok(res)
    }
}

pub fn api_version() -> Option<String> {
    let it = unsafe { sdk::cdecklink_create_iterator() };
    if it.is_null() {
        None
    } else {
        let c_str: &CStr = unsafe { CStr::from_ptr(sdk::cdecklink_api_version(it)) };
        let str = c_str.to_str().unwrap_or_default().to_string();
        unsafe { sdk::cdecklink_destroy_iterator(it); }
        Some(str)
    }
}

fn main() {
    println!("Hello, world!");

    let version = api_version().expect("Expected a version number");
    println!("Driver version: {}", version);

    let devices = get_devices().expect("list devices failed");
    println!("Found {} devices", devices.len());
    for device in devices {
        println!("{} - {}", device.model_name(), device.display_name());
    }
}
