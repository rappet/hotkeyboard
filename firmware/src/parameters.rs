use crate::Action;
use usbd_human_interface_device::page;

// Keyboard keys
//
// See
// https://docs.rs/usbd-human-interface-device/0.4.3/usbd_human_interface_device/page/enum.Keyboard.html
// for list of available keys
pub(crate) const LEFT_KEY: Action = Action::Keyboard(page::Keyboard::VolumeDown);
pub(crate) const RIGHT_KEY: Action = Action::Keyboard(page::Keyboard::VolumeUp);

// Consumer keys
//
// See
// https://docs.rs/usbd-human-interface-device/0.4.3/usbd_human_interface_device/page/enum.Consumer.html
// for list of available consumers
//pub(crate) const LEFT_KEY: Action = Action::Consumer(page::Consumer::PlayPause);
//pub(crate) const RIGHT_KEY: Action = Action::Consumer(page::Consumer::ScanNextTrack);

/*
 * USB specific metadata
 */
pub(crate) const USB_VID: u16 = 0x1209;
pub(crate) const USB_PID: u16 = 0x0001;

pub(crate) const USB_MANUFACTURER: &str = "rappet";
pub(crate) const USB_PRODUCT: &str = "Hotkey Keyboard";
pub(crate) const USB_SERIAL_NUMBER: &str = "0.1";
