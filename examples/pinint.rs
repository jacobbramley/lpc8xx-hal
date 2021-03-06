#![no_main]
#![no_std]

extern crate panic_rtt_target;

use lpc8xx_hal::{
    gpio::{direction::Output, GpioPin, Level},
    init_state::Enabled,
    pinint::{self, PININT0},
    pins::{PIO0_4, PIO1_1},
    Peripherals,
};

#[rtic::app(device = lpc8xx_hal::pac)]
const APP: () = {
    struct Resources {
        int: pinint::Interrupt<PININT0, PIO0_4, Enabled>,
        led: GpioPin<PIO1_1, Output>,
    }

    #[init]
    fn init(_: init::Context) -> init::LateResources {
        rtt_target::rtt_init_print!();

        let p = Peripherals::take().unwrap();

        let mut syscon = p.SYSCON.split();
        let gpio = p.GPIO.enable(&mut syscon.handle);
        let pinint = p.PININT.enable(&mut syscon.handle);

        let _button = p.pins.pio0_4.into_input_pin(gpio.tokens.pio0_4);
        let mut int = pinint
            .interrupts
            .pinint0
            .select::<PIO0_4>(&mut syscon.handle);
        int.enable_rising_edge();
        int.enable_falling_edge();

        let led = p
            .pins
            .pio1_1
            .into_output_pin(gpio.tokens.pio1_1, Level::High);

        init::LateResources { int, led }
    }

    #[task(binds = PIN_INT0, resources = [int, led])]
    fn pinint0(context: pinint0::Context) {
        let int = context.resources.int;
        let led = context.resources.led;

        led.toggle();

        int.clear_rising_edge_flag();
        int.clear_falling_edge_flag();
    }
};
