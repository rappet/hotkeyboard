use crate::Action;
use usbd_human_interface_device::page::Keyboard;

pub(crate) const LEFT_KEY: Action = Action::Keyboard(Keyboard::VolumeDown);
pub(crate) const RIGHT_KEY: Action = Action::Keyboard(Keyboard::VolumeUp);

/*
 * USB specific metadata
 */
pub(crate) const USB_VID: u16 = 0x1209;
pub(crate) const USB_PID: u16 = 0x0001;

pub(crate) const USB_MANUFACTURER: &str = "rappet";
pub(crate) const USB_PRODUCT: &str = "Hotkey Keyboard";
pub(crate) const USB_SERIAL_NUMBER: &str = "0.1";
