extern crate decklink_sdk;

use decklink_sdk::api_version;
use decklink_sdk::device::get_devices;

fn main() {
    let version = api_version().expect("Expected a version number");
    println!("Driver version: {}", version);

    let devices = get_devices().expect("list devices failed");
    println!("Found {} devices", devices.len());
    for device in devices {
        println!("{} - {}", device.model_name(), device.display_name());
    }
}
