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
use util::convert_string;
pub use util::SdkError;

pub fn api_version() -> Option<String> {
    let it = unsafe { sdk::cdecklink_create_decklink_api_information_instance() };
    if it.is_null() {
        None
    } else {
        let mut s = null();
        let str = unsafe { convert_string(sdk::cdecklink_api_version(it, &mut s), s) };
        unsafe { sdk::cdecklink_iterator_release(it) };
        str
    }
}
