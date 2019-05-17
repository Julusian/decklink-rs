use crate::sdk;

#[test]
fn test() {}

// input.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{get_devices, DecklinkDevice};

    fn get_device() -> DecklinkDevice {
        let mut devices = get_devices()
            .expect("Unable to list Decklink devices. The Decklink drivers may not be insalled.");
        //    let device = devices.first().expect("Could not find any Decklink devices");
        devices.swap_remove(2)
        //        devices[2]
    }

    #[test]
    fn test1() {
        let dev = get_device();
        let input = dev.input().unwrap();

        input.get_available_audio_sample_frame_count().expect("ok");
        input.get_available_video_frame_count().expect("ok");
    }

    #[test]
    fn test2() {
        let dev = get_device();
        let mut input = dev.input().unwrap();

        input.disable_audio_input();
        //        let result = unsafe { sdk::cdecklink_input_disable_audio_input(input.dev) };
        //        assert_eq!(0, result as u32);
    }
}
