#![feature(lang_items)]
#![no_std]
#![no_main]

use arduino_uno::prelude::*;

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    let mut led = pins.d13.into_output(&mut pins.ddr);

    loop {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(500);
    }
}

// No panic, just loop
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Exception handler is called to unwind panic
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

