#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate bitflags;
extern crate strum;
#[macro_use]
extern crate strum_macros;

#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::all
)]
// #[link(name = "decklink_c", kind = "static")]
mod sdk;

pub mod connectors;
pub mod device;
pub mod display_mode;
pub mod frame;
mod util;

use std::ptr::null;
use util::convert_and_release_c_string;
pub use util::SdkError;

/// Fetch the api version of the installed Decklink drivers.
///
/// If an error is returned, the drivers were not found on this system.
///
/// # Examples
///
/// ```
/// use decklink::api_version;
/// let version = api_version().unwrap();
/// println!("Version: {0}", version);
pub fn api_version() -> Result<String, SdkError> {
    let it = unsafe { sdk::cdecklink_create_decklink_api_information_instance() };
    if it.is_null() {
        Err(SdkError::FALSE)
    } else {
        let mut s = null();

        let result = unsafe { sdk::cdecklink_api_version(it, &mut s) };

        unsafe { sdk::cdecklink_iterator_release(it) };

        SdkError::result(result)?;

        let str = unsafe { convert_and_release_c_string(s) };

        Ok(str)
    }
}
