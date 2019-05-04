use num_traits::FromPrimitive;
use std::ffi::CStr;

// TODO - refactor the error type to abstract away weird errors?
#[derive(Debug, FromPrimitive)]
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
    pub fn from(value: i32) -> SdkError {
        SdkError::from_i32(value).unwrap_or(SdkError::FALSE)
    }
    pub fn is_false(value: i32) -> bool {
        value == (SdkError::FALSE as i32)
    }
    pub fn is_ok(value: i32) -> bool { value == 0 }
    pub fn succeeded(r: i32) -> bool {
        r >= 0
    }
}

pub unsafe fn convert_string(ptr: *const ::std::os::raw::c_char) -> String {
    let str = CStr::from_ptr(ptr).to_str().unwrap_or_default().to_string();
    crate::sdk::cdecklink_free_string(ptr);
    str
}
