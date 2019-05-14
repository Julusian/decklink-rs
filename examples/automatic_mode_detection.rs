extern crate decklink_sdk;
#[macro_use]
extern crate text_io;

use decklink_sdk::device::get_devices;
use decklink_sdk::device::input::{
    DeckLinkVideoInputCallback, DecklinkAudioInputPacket, DecklinkDetectedVideoInputFormatFlags,
    DecklinkInputDevice, DecklinkVideoInputFlags, DecklinkVideoInputFormatChangedEvents,
};
use decklink_sdk::display_mode::{DecklinkDisplayMode, DecklinkDisplayModeId};
use decklink_sdk::frame::{DecklinkPixelFormat, DecklinkVideoFrame};
use std::sync::{Arc, Mutex};

struct InputCallback {
    input: Arc<Mutex<DecklinkInputDevice>>,
}
impl DeckLinkVideoInputCallback for InputCallback {
    fn input_frame_arrived(
        &self,
        _video_frame: Option<DecklinkVideoFrame>,
        _audio_packet: Option<DecklinkAudioInputPacket>,
    ) -> bool {
        true
    }

    fn video_input_format_changed(
        &self,
        events: DecklinkVideoInputFormatChangedEvents,
        display_mode: Option<DecklinkDisplayMode>,
        signal_flags: DecklinkDetectedVideoInputFormatFlags,
    ) -> bool {
        if let Some(display_mode) = display_mode {
            let mut pixel_format = DecklinkPixelFormat::Format10BitYUV;

            if events.contains(DecklinkVideoInputFormatChangedEvents::FIELD_DOMINANCE) {
                println!(
                    "Input field dominance changed to: {:?}",
                    display_mode.field_dominance()
                );
            }

            if events.contains(DecklinkVideoInputFormatChangedEvents::COLORSPACE) {
                print!("Input color space changed to: ");

                if signal_flags == DecklinkDetectedVideoInputFormatFlags::YCBCR422 {
                    println!("YCbCr422");
                    pixel_format = DecklinkPixelFormat::Format10BitYUV;
                } else if signal_flags == DecklinkDetectedVideoInputFormatFlags::RGB444 {
                    println!("RGB444");
                    pixel_format = DecklinkPixelFormat::Format10BitRGB;
                }
            }

            if events.contains(DecklinkVideoInputFormatChangedEvents::DISPLAY_MODE) {
                println!(
                    "Input display mode changed to: {:?}",
                    display_mode.name().unwrap_or_default()
                );
            }

            // Restart streams
            let input = self.input.lock().unwrap();
            input.pause_streams().expect("Cannot pause streams");
            input
                .enable_video_input(
                    display_mode.mode(),
                    pixel_format,
                    DecklinkVideoInputFlags::ENABLE_FORMAT_DETECTION,
                )
                .expect("Coult not switch video mode");
            input.flush_streams().expect("Cannot flush streams");
            input.start_streams().expect("Cannot start streams");

            true
        } else {
            false
        }
    }
}

fn main() {
    let devices = get_devices()
        .expect("Unable to list Decklink devices. The Decklink drivers may not be insalled.");
    //    let device = devices.first().expect("Could not find any Decklink devices");
    let device = &devices[2];

    let attributes = device
        .get_attributes()
        .expect("Could not obtain Decklink attributes");

    println!("Device: {}", device.display_name().unwrap_or_default());

    if !attributes.supports_input_format_detection().unwrap() {
        panic!("Device does not support automatic mode detection");
    }

    let input = Arc::new(Mutex::new(
        device.input().expect("Unable to open device as input"),
    ));

    let callback = Arc::new(InputCallback {
        input: input.clone(),
    });

    {
        let mut input2 = input.lock().unwrap();
        input2
            .set_callback(callback)
            .expect("Could not set input callback");

        input2
            .enable_video_input(
                DecklinkDisplayModeId::HD1080i50,
                DecklinkPixelFormat::Format10BitYUV,
                DecklinkVideoInputFlags::ENABLE_FORMAT_DETECTION,
            )
            .expect("Could not enable video input");

        println!("Starting streams");

        input2.start_streams().expect("Could not start capture");
    }

    println!("Press enter to continue");
    let _s: String = read!();

    // Cleanup happens during object destruction
}
