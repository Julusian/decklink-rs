extern crate decklink_sdk;
#[macro_use]
extern crate text_io;

use decklink_sdk::device::get_devices;
use decklink_sdk::device::output::DecklinkOutputFrameFlags;
use decklink_sdk::frame::{DecklinkFrameFlags, DecklinkPixelFormat};

fn main() {
    let device = {
        let mut devices = get_devices().expect("list devices failed");
        println!("Found {} devices", devices.len());
        for i in 0..devices.len() {
            println!("{}: {}", i, devices[i].display_name());
        }

        let index: usize = read!();
        if index >= devices.len() {
            println!("Invalid device index");
            return;
        }

        devices.swap_remove(index)
    };

    println!("Selected device: {}\n", device.display_name());

    let output = match device.output() {
        None => {
            println!("Failed to create device output");
            return;
        }
        Some(o) => o,
    };

    let mode = {
        let mut supported_modes = output
            .display_modes()
            .expect("Failed to list display modes");
        for i in 0..supported_modes.len() {
            println!("{}: {}", i, supported_modes[i].name());
        }

        let index: usize = read!();
        if index >= supported_modes.len() {
            println!("Invalid mode index");
            return;
        }

        supported_modes.swap_remove(index)
    };

    let frame = output
        .create_video_frame(
            mode.width() as i32,
            mode.height() as i32,
            (mode.width() * 4) as i32,
            DecklinkPixelFormat::Format8BitBGRA,
            DecklinkFrameFlags::empty(),
        )
        .expect("Failed to create video frame");

    // TODO - fill bytes
    let bytes = vec![120u8; (mode.width() * mode.height() * 4) as usize];
    if !frame.base().set_bytes(&bytes) {
        println!("Failed to set frame bytes");
        return;
    }

    output
        .enable_video_output(mode.mode(), DecklinkOutputFrameFlags::empty())
        .expect("Failed to enable video output");
    output
        .display_video_frame_sync(frame.base())
        .expect("Failed to display frame");

    println!("Press enter to continue");
    let _s: String = read!();

    // All done
}
