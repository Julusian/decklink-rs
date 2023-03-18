extern crate decklink;
#[macro_use]
extern crate text_io;

use decklink::device::get_devices;
use decklink::device::notification::{
    DeckLinkNotificationCallback, DecklinkDeviceNotificationExt, NotificationTopic,
};
use decklink::device::status::{DecklinkDeviceStatus, DecklinkStatusId};
use std::sync::Arc;

struct NotificationCallback {
    status: Arc<DecklinkDeviceStatus>,
}
impl DeckLinkNotificationCallback for NotificationCallback {
    fn notify_status(&self, id: DecklinkStatusId) -> bool {
        print_status(&self.status, id);
        true
    }
}

fn print_status(status: &DecklinkDeviceStatus, id: DecklinkStatusId) {
    /*
     * Failed to retrieve the status value. Don't complain as this is
     * expected for different hardware configurations.
     *
     * e.g.
     * A device that doesn't support automatic mode detection will fail
     * a request for DecklinkStatusId::DetectedVideoInputMode.
     */
    match id {
        DecklinkStatusId::DetectedVideoInputMode => {
            if let Ok(value) = status.detected_video_input_mode() {
                print_line(id, format!("{:?}", value))
            }
        }
        DecklinkStatusId::DetectedVideoInputFlags => {
            if let Ok(value) = status.detected_video_input_flags() {
                print_line(id, format!("{:08X}", value))
            }
        }
        DecklinkStatusId::CurrentVideoInputMode => {
            if let Ok(value) = status.current_video_input_mode() {
                print_line(id, format!("{:?}", value))
            }
        }
        DecklinkStatusId::CurrentVideoInputPixelFormat => {
            if let Ok(value) = status.current_video_input_pixel_format() {
                print_line(id, format!("{:?}", value))
            }
        }
        DecklinkStatusId::CurrentVideoInputFlags => {
            if let Ok(value) = status.current_video_input_flags() {
                print_line(id, format!("{:08X}", value))
            }
        }
        DecklinkStatusId::CurrentVideoOutputMode => {
            if let Ok(value) = status.current_video_output_mode() {
                print_line(id, format!("{:?}", value))
            }
        }
        DecklinkStatusId::CurrentVideoOutputFlags => {
            if let Ok(value) = status.current_video_output_flags() {
                print_line(id, format!("{:08X}", value))
            }
        }
        DecklinkStatusId::PCIExpressLinkWidth => {
            if let Ok(value) = status.pci_express_link_width() {
                print_line(id, format!("{:X}", value))
            }
        }
        DecklinkStatusId::PCIExpressLinkSpeed => {
            if let Ok(value) = status.pci_express_link_speed() {
                print_line(id, format!("{}", value))
            }
        }
        DecklinkStatusId::LastVideoOutputPixelFormat => {
            if let Ok(value) = status.last_video_output_pixel_format() {
                print_line(id, format!("{:?}", value))
            }
        }
        DecklinkStatusId::ReferenceSignalMode => {
            if let Ok(value) = status.reference_signal_mode() {
                print_line(id, format!("{:?}", value))
            }
        }
        DecklinkStatusId::ReferenceSignalFlags => {
            if let Ok(value) = status.reference_signal_flags() {
                print_line(id, format!("{:08X}", value))
            }
        }
        DecklinkStatusId::DuplexMode => {
            if let Ok(value) = status.duplex_mode() {
                print_line(id, format!("{:?}", value))
            }
        }
        DecklinkStatusId::Busy => {
            if let Ok(value) = status.busy() {
                print_line(id, format!("{:08X}", value))
            }
        }
        DecklinkStatusId::InterchangeablePanelType => {
            if let Ok(value) = status.interchangeable_panel_type() {
                print_line(id, format!("{:08X}", value))
            }
        }
        DecklinkStatusId::VideoInputSignalLocked => {
            if let Ok(value) = status.video_input_signal_locked() {
                print_line(id, format!("{:?}", value))
            }
        }
        DecklinkStatusId::ReferenceSignalLocked => {
            if let Ok(value) = status.reference_signal_locked() {
                print_line(id, format!("{:?}", value))
            }
        }
        DecklinkStatusId::ReceivedEDID => {
            if let Ok(value) = status.received_edid() {
                print_line(id, format!("{:?}", value))
            }
        }
    }
}

fn print_line(id: DecklinkStatusId, value: String) {
    println!("{0: <40} {1}", format!("{:?}:", id), value);
}

fn main() {
    let devices = get_devices()
        .expect("Unable to list Decklink devices. The Decklink drivers may not be insalled.");
    //    let device = devices.first().expect("Could not find any Decklink devices");
    let device = &devices[0];

    let status = Arc::new(
        device
            .get_status()
            .expect("Could not obtain Decklink status object"),
    );

    // Print general status values
    print_status(&status, DecklinkStatusId::Busy);
    print_status(&status, DecklinkStatusId::DuplexMode);
    print_status(&status, DecklinkStatusId::PCIExpressLinkWidth);
    print_status(&status, DecklinkStatusId::PCIExpressLinkSpeed);

    // Print video input status values
    print_status(&status, DecklinkStatusId::VideoInputSignalLocked);
    print_status(&status, DecklinkStatusId::DetectedVideoInputMode);
    print_status(&status, DecklinkStatusId::DetectedVideoInputFlags);
    print_status(&status, DecklinkStatusId::CurrentVideoInputMode);
    print_status(&status, DecklinkStatusId::CurrentVideoInputFlags);
    print_status(&status, DecklinkStatusId::CurrentVideoInputPixelFormat);

    // Print video output status values
    print_status(&status, DecklinkStatusId::CurrentVideoOutputMode);
    print_status(&status, DecklinkStatusId::CurrentVideoOutputFlags);
    print_status(&status, DecklinkStatusId::LastVideoOutputPixelFormat);
    print_status(&status, DecklinkStatusId::ReferenceSignalLocked);
    print_status(&status, DecklinkStatusId::ReferenceSignalMode);
    print_status(&status, DecklinkStatusId::ReceivedEDID);

    let notification = device
        .get_notification()
        .expect("Could not obtain Decklink notification object");

    let callback = Arc::new(NotificationCallback {
        status: status.clone(),
    });

    // Register for notification changes
    // The registration is only valid for as long as handle is alive
    let _handle = notification
        .subscribe(NotificationTopic::StatusChanged, callback.clone())
        .expect("Failed to subscribe to notifications");

    println!("Press enter to continue");
    let _s: String = read!();

    // Cleanup happens during object destruction
}
