extern crate decklink;
#[macro_use]
extern crate text_io;

use decklink::device::output::{
    DeckLinkVideoOutputCallback, DecklinkOutputDeviceVideoScheduled,
    DecklinkOutputFrameCompletionResult, DecklinkVideoOutputFlags,
};
use decklink::device::DecklinkDisplayModeSupport;
use decklink::device::{get_devices, DecklinkDeviceDisplayModes};
use decklink::display_mode::DecklinkDisplayModeId;
use decklink::frame::{
    DecklinkFrameBase, DecklinkFrameFlags, DecklinkPixelFormat, DecklinkVideoFrame,
    DecklinkVideoMutableFrame,
};
use decklink::SdkError;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex, Weak};

struct OutputCallback {
    output: Weak<Mutex<Box<dyn DecklinkOutputDeviceVideoScheduled>>>,
    duration: i64,
    timescale: i64,

    scheduled: AtomicI64,
}

impl DeckLinkVideoOutputCallback for OutputCallback {
    fn schedule_frame_completed_callback(
        &self,
        frame: Option<DecklinkVideoFrame>,
        _result: DecklinkOutputFrameCompletionResult,
    ) -> bool {
        self.schedule_next_frame(&frame.unwrap()).is_ok()
    }

    fn playback_stopped(&self) -> bool {
        true
    }
}
impl OutputCallback {
    fn convert_frame_number_to_timecode(&self, frame_count: i64) -> (i64, i64, i64, i64) {
        let max_fps = self.timescale / 1000;
        let mut remaining_frame_count = frame_count;

        let is_drop_frame = false; // TODO
        if is_drop_frame && self.duration == 1001 {
            // TODO
        }

        let frames = remaining_frame_count % max_fps;
        remaining_frame_count /= max_fps;
        let seconds = remaining_frame_count % 60;
        remaining_frame_count /= 60;
        let minutes = remaining_frame_count % 60;
        remaining_frame_count /= 60;
        let hours = remaining_frame_count % 24;

        return (hours, minutes, seconds, frames);
    }

    fn schedule_next_frame(&self, frame: &dyn DecklinkFrameBase) -> Result<(), SdkError> {
        if let Some(output) = self.output.upgrade() {
            let num = self.scheduled.fetch_add(1, Ordering::SeqCst);
            let _timecode = self.convert_frame_number_to_timecode(num);

            output
                .lock()
                .unwrap()
                .schedule_frame(frame, num * self.duration, self.duration)
        } else {
            Err(SdkError::HANDLE)
        }
    }
}

fn main() {
    let mode = DecklinkDisplayModeId::HD1080i50;
    let pixel_format = DecklinkPixelFormat::Format10BitYUV;
    let output_flags = DecklinkVideoOutputFlags::RP188;

    let devices = get_devices()
        .expect("Unable to list Decklink devices. The Decklink drivers may not be insalled.");
    //    let device = devices.first().expect("Could not find any Decklink devices");
    let device = &devices[1];

    let output = Arc::new(
        device
            .output()
            .expect("Could not obtain the Decklink output"),
    );

    let sm = output
        .does_support_video_mode(mode, pixel_format, output_flags)
        .expect("Could not check if output supports mode");
    if sm.0 != DecklinkDisplayModeSupport::Supported {
        println!("Video mode is not supported");
        return;
    }

    let display_mode = sm.1.unwrap();
    let fps = display_mode.framerate().expect("Could not get framerate");

    let mut frame = DecklinkVideoMutableFrame::create(
        display_mode.width(),
        display_mode.height(),
        display_mode.width() * 4,
        pixel_format,
        DecklinkFrameFlags::empty(),
    );

    {
        // let blue_data = [0x40aa298, 0x2a8a62a8, 0x298aa040, 0x2a8102a8];
        let blue_data = [
            0x04, 0x0a, 0xa2, 0x98, 0x2a, 0x8a, 0x62, 0xa8, 0x29, 0x8a, 0xa0, 0x40, 0x2a, 0x81,
            0x02, 0xa8,
        ];

        let mut frame_data = Vec::new();
        frame_data.reserve((display_mode.width() * display_mode.height()) as usize);
        for i in 0..frame_data.len() {
            frame_data[i] = blue_data[i % blue_data.len()];
        }
        frame.set_bytes(frame_data).expect("set bytes failed");
    }

    let output_scheduled = Arc::new(Mutex::new(
        output
            .enable_video_output_scheduled(mode, output_flags, fps.1)
            .expect("Could not setup scheduled output mode"),
    ));

    let callback = Arc::new(OutputCallback {
        output: Arc::downgrade(&output_scheduled),
        duration: fps.0,
        timescale: fps.1,
        scheduled: AtomicI64::new(0),
    });

    for _ in 0..4 {
        callback
            .schedule_next_frame(&frame)
            .expect("Could not schedule video frame");
    }

    let mut output_scheduled2 = output_scheduled.lock().unwrap();
    output_scheduled2
        .set_callback(Some(callback))
        .expect("Failed to set output callback");

    output_scheduled2
        .start_playback(0, 1.0)
        .expect("Could not start playback");

    println!("Press enter to continue");
    let _s: String = read!();

    // Cleanup happens during object destruction
}
