#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use defmt::info;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use stm32f0xx_hal::{
    pac,
    prelude::*,
    rcc::USBClockSource,
    time::Hertz,
    timers::Timer,
    usb::{Peripheral, UsbBus as StmUsbBus},
};
use usb_device::prelude::*;
use usbd_human_interface_device::{
    device::{
        consumer::{ConsumerControlConfig, MultipleConsumerReport},
        keyboard::BootKeyboardConfig,
    },
    page::{Consumer, Keyboard},
    usb_class::UsbHidClassBuilder,
};

mod parameters;

#[derive(Copy, Clone)]
// Allow dead code as certain action might or might not be used depending on config
#[allow(dead_code)]
pub(crate) enum Action {
    Keyboard(Keyboard),
    Consumer(Consumer),
}

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();

    // Remap so USB works
    stm32f0xx_hal::usb::remap_pins(&mut dp.RCC, &mut dp.SYSCFG);

    let mut rcc = dp
        .RCC
        .configure()
        .hsi48()
        .enable_crs(dp.CRS)
        .sysclk(48.mhz())
        .pclk(24.mhz())
        .usbsrc(USBClockSource::HSI48)
        .freeze(&mut dp.FLASH);

    // Configure the on-board LED (LD3, green)
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);
    //let mut led = cortex_m::interrupt::free(|cs| gpiob.pb3.into_push_pull_output(cs));
    let left_key = cortex_m::interrupt::free(|cs| gpioa.pa5.into_floating_input(cs));
    let right_key = cortex_m::interrupt::free(|cs| gpiob.pb8.into_pull_down_input(cs));

    let mut timer = Timer::tim1(dp.TIM1, Hertz(1000), &mut rcc);

    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,
        pin_dp: gpioa.pa12,
    };

    let usb_allocator = StmUsbBus::new(usb);

    let mut keyboard = UsbHidClassBuilder::new()
        .add_device(BootKeyboardConfig::default())
        .build(&usb_allocator);
    let mut consumer_device = UsbHidClassBuilder::new()
        .add_device(ConsumerControlConfig::default())
        .build(&usb_allocator);

    let mut usb_dev = UsbDeviceBuilder::new(
        &usb_allocator,
        UsbVidPid(parameters::USB_VID, parameters::USB_PID),
    )
    .manufacturer(parameters::USB_MANUFACTURER)
    .product(parameters::USB_PRODUCT)
    .serial_number(parameters::USB_SERIAL_NUMBER)
    .build();

    timer.start(Hertz(1000));

    let mut i = 0u32;

    info!("start main loop");

    loop {
        i = i.wrapping_add(1);

        if i % 10 == 0 {
            let left = left_key.is_high().unwrap();
            let right = right_key.is_high().unwrap();

            let keys = [
                match parameters::LEFT_KEY {
                    Action::Keyboard(key) if left => key,
                    _ => Keyboard::NoEventIndicated,
                },
                match parameters::RIGHT_KEY {
                    Action::Keyboard(key) if right => key,
                    _ => Keyboard::NoEventIndicated,
                },
            ];

            keyboard.device().write_report(keys).ok();

            let consumers = [
                match parameters::LEFT_KEY {
                    Action::Consumer(consumer) if left => consumer,
                    _ => Consumer::Unassigned,
                },
                match parameters::RIGHT_KEY {
                    Action::Consumer(consumer) if right => consumer,
                    _ => Consumer::Unassigned,
                },
                Consumer::Unassigned,
                Consumer::Unassigned,
            ];

            consumer_device
                .device()
                .write_report(&MultipleConsumerReport { codes: consumers })
                .ok();
        }

        keyboard.tick().ok();
        consumer_device.tick().ok();

        if nb::block!(timer.wait()).is_ok() {
            keyboard.tick().unwrap();
        }

        if usb_dev.poll(&mut [&mut keyboard, &mut consumer_device]) {
            let _ = keyboard.device().read_report();
        }
    }
}
