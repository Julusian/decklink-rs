#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate bitflags;

#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::all
)]
#[link(name = "decklink_c", kind = "static")]
mod sdk;

pub mod device;
pub mod display_mode;
pub mod frame;
mod util;

use util::convert_string;
pub use util::SdkError;

pub fn api_version() -> Option<String> {
    let it = unsafe { sdk::cdecklink_create_iterator() };
    if it.is_null() {
        None
    } else {
        let str = unsafe { convert_string(sdk::cdecklink_api_version(it)) };
        unsafe {
            sdk::cdecklink_destroy_iterator(it);
        }
        Some(str)
    }
}
