extern crate decklink;

use decklink::api_version;
use decklink::connectors::DecklinkVideoConnection;
use decklink::device::output::DecklinkVideoOutputFlags;
use decklink::device::DecklinkDisplayModeSupport;
use decklink::device::{get_devices, DecklinkDevice, DecklinkDeviceDisplayModes};
use decklink::display_mode::DecklinkDisplayMode;
use decklink::frame::DecklinkPixelFormat;
use strum::IntoEnumIterator;

fn main() {
    if let Ok(version) = api_version() {
        println!("Driver version: {}", version);
    } else {
        println!("Failed to get decklink driver version");
        return;
    }

    match get_devices() {
        Err(_) => println!(
            "A DeckLink iterator could not be created.  The DeckLink drivers may not be installed."
        ),
        Ok(devices) => {
            if devices.len() == 0 {
                println!("No Blackmagic Design devices were found.\n");
            } else {
                for device in devices {
                    if let Some(name) = device.model_name() {
                        println!("=============== {} ===============\n", name);
                    }

                    print_attributes(&device);
                    println!();

                    print_output_modes(&device);
                    println!();

                    print_input_modes(&device);
                    println!();

                    print_capabilities(&device);
                    println!();
                }
            }
        }
    }
}

fn print_attributes(device: &DecklinkDevice) {
    match device.get_attributes() {
        Err(e) => println!("Could not obtain the device attributes - result = {:?}", e),
        Ok(attributes) => {
            println!("Attribute list:");

            let print_col = |name: &str, val: &str| println!(" {0: <40} {1: <10}", name, val);
            let print_col_int = |name: &str, val: i64| println!(" {0: <40} {1: <10}", name, val);
            let print_col_bool =
                |name: &str, val: bool| print_col(name, if val { "Yes" } else { "No" });

            if let Ok(supported) = attributes.has_serial_port() {
                print_col_bool("Serial port present ?", supported);
                if supported {
                    let name = attributes
                        .serial_port_device_name()
                        .unwrap_or_else(|_| "Unknown".to_string());
                    print_col("Serial port name: ", &name);
                }
            } else {
                print_col("Serial port present ?", "Unknown");
            }

            if let Ok(id) = attributes.persistent_id() {
                print_col_int("Device Persistent ID:", id);
            } else {
                print_col("Device Persistent ID:", "Not Supported on this device");
            }

            if let Ok(id) = attributes.topological_id() {
                // TODO - this doesnt match c++
                print_col_int("Device Topological ID:", id);
            } else {
                print_col("Device Topological ID:", "Not Supported on this device");
            }

            if let Ok(count) = attributes.number_of_sub_devices() {
                print_col_int("Number of sub-devices:", count);
                if count != 0 {
                    if let Ok(index) = attributes.sub_device_index() {
                        print_col_int("Sub-device index:", index);
                    } else {
                        print_col("Sub-device index:", "Unknown");
                    }
                }
            } else {
                print_col("Number of sub-devices:", "Unknown");
            }

            if let Ok(count) = attributes.maximum_audio_channels() {
                print_col_int("Number of audio channels:", count);
            } else {
                print_col("Number of audio channels:", "Unknown");
            }

            if let Ok(supported) = attributes.supports_input_format_detection() {
                print_col_bool("Input mode detection supported ?", supported);
            } else {
                print_col("Input mode detection supported ?", "Unknown");
            }

            if let Ok(supported) = attributes.supports_full_duplex() {
                print_col_bool("Full duplex operation supported ?", supported);
            } else {
                print_col("Full duplex operation supported ?", "Unknown");
            }

            if let Ok(supported) = attributes.supports_internal_keying() {
                print_col_bool("Internal keying supported ?", supported);
            } else {
                print_col("Internal keying supported ?", "Unknown");
            }

            if let Ok(supported) = attributes.supports_external_keying() {
                print_col_bool("External keying supported ?", supported);
            } else {
                print_col("External keying supported ?", "Unknown");
            }

            if let Ok(supported) = attributes.supports_hd_keying() {
                print_col_bool("HD-mode keying supported ?", supported);
            } else {
                print_col("HD-mode keying supported ?", "Unknown");
            }

            //
        }
    }
}

fn print_modes<T>(
    modes: Vec<DecklinkDisplayMode>,
    dev: &dyn DecklinkDeviceDisplayModes<T>,
    flags: T,
) where
    T: Copy,
{
    for mode in modes {
        let name = mode.name().unwrap_or_else(|| "Unknown".to_string());
        let width = mode.width();
        let height = mode.height();
        let fps = if let Some(fps) = mode.framerate() {
            (fps.1 as f64) / (fps.0 as f64)
        } else {
            0.0
        };

        print!(
            "{0: <20} \t {1} x {2}  \t {3: <7.3} FPS\t",
            name, width, height, fps
        );

        for format in DecklinkPixelFormat::iter() {
            let supported = {
                let v = dev.does_support_video_mode(mode.mode(), format, flags);
                match v {
                    Err(_) => DecklinkDisplayModeSupport::NotSupported,
                    Ok(v) => v.0,
                }
            };

            if supported == DecklinkDisplayModeSupport::NotSupported {
                print!("------\t\t");
            } else {
                print!("{:?}\t", format);
            }
        }

        println!();
    }
}

fn print_output_modes(device: &DecklinkDevice) {
    if let Some(output) = device.output() {
        if let Ok(modes) = output.display_modes() {
            println!("Supported video output display modes and pixel formats:");

            print_modes(modes, &output, DecklinkVideoOutputFlags::empty());
        } else {
            println!("Could not obtain supported display mode list");
        }
    } else {
        println!("Could not obtain the device output");
    }
}

fn print_input_modes(_device: &DecklinkDevice) {
    //    if let Some(input) = device.input() {
    //        if let Ok(modes) = output.display_modes() {
    //            println!("Supported video input display modes and pixel formats:");
    //
    //            print_modes(modes, &input, DecklinkVideoInputFlags::empty());
    //        } else {
    //            println!("Could not obtain supported display mode list");
    //        }
    //    } else {
    //        println!("Could not obtain the device input");
    //    }
}

fn print_capabilities(device: &DecklinkDevice) {
    let list_supported_ports = |ports: DecklinkVideoConnection| {
        let mut items = Vec::new();

        if ports.contains(DecklinkVideoConnection::SDI) {
            items.push("SDI");
        }
        if ports.contains(DecklinkVideoConnection::HDMI) {
            items.push("HDMI");
        }
        if ports.contains(DecklinkVideoConnection::OPTICAL_SDI) {
            items.push("Optical SDI");
        }
        if ports.contains(DecklinkVideoConnection::COMPONENT) {
            items.push("Component");
        }
        if ports.contains(DecklinkVideoConnection::COMPOSITE) {
            items.push("Composite");
        }
        if ports.contains(DecklinkVideoConnection::SVIDEO) {
            items.push("SVideo");
        }
        items
    };

    match device.get_attributes() {
        Err(e) => println!("Could not obtain the device attributes - result = {:?}", e),
        Ok(attributes) => {
            println!("Supported video output connections:");
            if let Ok(ports) = attributes.video_output_connections() {
                println!("{}", list_supported_ports(ports).join(", "));
            } else {
                println!("Could not obtain the list of output ports");
            }
            println!();

            println!("Supported video input connections:");
            if let Ok(ports) = attributes.video_input_connections() {
                println!("{}", list_supported_ports(ports).join(", "));
            } else {
                println!("Could not obtain the list of input ports");
            }
            println!();
        }
    }
}
