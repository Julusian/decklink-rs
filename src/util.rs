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
        Self::from_i32(value).unwrap_or(SdkError::FALSE)
    }
    pub fn is_false(value: i32) -> bool {
        value == (SdkError::FALSE as i32)
    }
    pub fn is_ok(value: i32) -> bool {
        value == 0
    }

    pub fn result<T>(r: i32) -> Result<T, SdkError>
    where
        T: Default,
    {
        Self::result_or_else(r, || Default::default())
    }
    pub fn result_or<T>(r: i32, def: T) -> Result<T, SdkError> {
        if Self::is_ok(r) {
            Ok(def)
        } else {
            Err(Self::from(r))
        }
    }
    pub fn result_or_else<T, F: FnOnce() -> T>(r: i32, ok: F) -> Result<T, SdkError> {
        if Self::is_ok(r) {
            Ok(ok())
        } else {
            Err(Self::from(r))
        }
    }
}

pub unsafe fn convert_string_inner(ptr: *const ::std::os::raw::c_char) -> String {
    let str = CStr::from_ptr(ptr).to_str().unwrap_or_default().to_string();
    crate::sdk::cdecklink_free_string(ptr);
    str
}

pub unsafe fn convert_string(res: i32, ptr: *const ::std::os::raw::c_char) -> Option<String> {
    if res == 0 {
        Some(convert_string_inner(ptr))
    } else {
        None
    }
}
