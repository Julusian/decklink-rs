extern crate decklink;
#[macro_use]
extern crate text_io;

use decklink::device::output::{DecklinkOutputDevice, DecklinkVideoOutputFlags};
use decklink::device::DecklinkDeviceDisplayModes;
use decklink::device::{get_devices, DecklinkDevice};
use decklink::display_mode::DecklinkDisplayMode;
use decklink::frame::{DecklinkFrameFlags, DecklinkPixelFormat, DecklinkVideoMutableFrame};

fn select_output_and_format() -> Option<(DecklinkDevice, DecklinkOutputDevice, DecklinkDisplayMode)>
{
    let device = {
        let mut devices = get_devices().expect("list devices failed");
        println!("Found {} devices", devices.len());
        for i in 0..devices.len() {
            println!(
                "{}: {}",
                i,
                devices[i]
                    .display_name()
                    .unwrap_or_else(|| "Unknown".to_string())
            );
        }

        let index: usize = text_io::read!();
        if index >= devices.len() {
            println!("Invalid device index");
            return None;
        }

        devices.swap_remove(index)
    };

    println!(
        "Selected device: {}\n",
        device
            .display_name()
            .unwrap_or_else(|| "Unknown".to_string())
    );

    let output = match device.output() {
        None => {
            println!("Failed to create device output");
            return None;
        }
        Some(o) => o,
    };

    let mode = {
        let mut supported_modes = output
            .display_modes()
            .expect("Failed to list display modes");
        for i in 0..supported_modes.len() {
            println!(
                "{}: {}",
                i,
                supported_modes[i]
                    .name()
                    .unwrap_or_else(|| "Unknown".to_string())
            );
        }

        let index: usize = read!();
        if index >= supported_modes.len() {
            println!("Invalid mode index");
            return None;
        }

        supported_modes.swap_remove(index)
    };

    Some((device, output, mode))
}

fn main() {
    if let Some((_device, output, mode)) = select_output_and_format() {
        let mut frame = Box::new(DecklinkVideoMutableFrame::create(
            mode.width(),
            mode.height(),
            mode.width() * 4,
            DecklinkPixelFormat::Format8BitBGRA,
            DecklinkFrameFlags::empty(),
        ));

        let bytes = vec![120u8; (mode.width() * mode.height() * 4) as usize];
        if frame.copy_bytes(&bytes).is_err() {
            println!("Failed to set frame bytes");
            return;
        }

        let video_output = output
            .enable_video_output_sync(mode.mode(), DecklinkVideoOutputFlags::empty())
            .expect("Failed to enable video output");

        video_output
            .display_custom_frame(frame)
            .expect("Failed to display frame");

        println!("Press enter to continue");
        let _s: String = read!();

        // All done
    }
}
