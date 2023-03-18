use num_traits::FromPrimitive;
use std::ffi::CStr;

// TODO - refactor the error type to abstract away weird errors?
#[derive(Debug, FromPrimitive)]
#[allow(overflowing_literals)]
pub enum SdkError {
    FALSE = 0x0000_0001,
    UNEXPECTED = -0x0000_FFFF,
    NOTIMPL = -0x0000_0001,
    OUTOFMEMORY = -0x0000_0002,
    INVALIDARG = -0x0000_0003,
    NOINTERFACE = -0x0000_0004,
    POINTER = -0x0000_0005,
    HANDLE = -0x0000_0006,
    ABORT = -0x0000_0007,
    FAIL = -0x0000_0008,
    ACCESSDENIED = -0x0009,
}

impl SdkError {
    #[allow(overflowing_literals)]
    pub(crate) fn from(value: i32) -> SdkError {
        Self::from_i32(value).unwrap_or(SdkError::FALSE)
    }
    pub(crate) fn is_false(value: i32) -> bool {
        value == (SdkError::FALSE as i32)
    }
    pub(crate) fn is_ok(value: i32) -> bool {
        value == 0
    }

    pub(crate) fn result<T>(r: i32) -> Result<T, SdkError>
    where
        T: Default,
    {
        Self::result_or_else(r, Default::default)
    }
    pub(crate) fn result_or<T>(r: i32, def: T) -> Result<T, SdkError> {
        if Self::is_ok(r) {
            Ok(def)
        } else {
            Err(Self::from(r))
        }
    }
    pub(crate) fn result_or_else<T, F: FnOnce() -> T>(r: i32, ok: F) -> Result<T, SdkError> {
        if Self::is_ok(r) {
            Ok(ok())
        } else {
            Err(Self::from(r))
        }
    }
}

pub(crate) unsafe fn convert_and_release_c_string(ptr: *const ::std::os::raw::c_char) -> String {
    let str = CStr::from_ptr(ptr).to_str().unwrap_or_default().to_string();
    crate::sdk::cdecklink_free_string(ptr);
    str
}

pub(crate) unsafe fn convert_string(
    res: i32,
    ptr: *const ::std::os::raw::c_char,
) -> Option<String> {
    if res == 0 {
        Some(convert_and_release_c_string(ptr))
    } else {
        None
    }
}
