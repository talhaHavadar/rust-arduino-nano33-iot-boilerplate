#![no_std]
#![no_main]
extern crate arduino_nano33iot as hal;

use hal::clock::GenericClockController;
use hal::prelude::*;
use rtic::app;

#[app(device = crate::hal::pac, peripherals = false)]
const APP: () = {
    struct Resources {
        red_led: hal::gpio::Pa17<hal::gpio::Output<hal::gpio::OpenDrain>>,
        timer: hal::timer::TimerCounter3,
    }

    #[task(binds = TC3, resources = [timer, red_led])]
    fn tc3(c: tc3::Context) {
        if c.resources.timer.wait().is_ok() {
            c.resources.red_led.toggle();
        }
    }

    #[init]
    fn init(_c: init::Context) -> init::LateResources {
        let mut device = hal::pac::Peripherals::take().unwrap();

        let mut clocks = GenericClockController::with_internal_32kosc(
            device.GCLK,
            &mut device.PM,
            &mut device.SYSCTRL,
            &mut device.NVMCTRL,
        );
        let gclk1 = clocks.gclk1();
        let mut pins = hal::Pins::new(device.PORT);

        let mut tc3 = hal::timer::TimerCounter::tc3_(
            &clocks.tcc2_tc3(&gclk1).unwrap(),
            device.TC3,
            &mut device.PM,
        );
        tc3.start(1.hz());
        tc3.enable_interrupt();

        let mut led = pins.led_sck.into_open_drain_output(&mut pins.port);
        led.set_high().unwrap();

        init::LateResources {
            red_led: led,
            timer: tc3,
        }
    }
};
