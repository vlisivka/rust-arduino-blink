// Standard library is not available.
#![no_std]

// No typical main function, with _start(), constructors, destructors, etc.
#![no_main]

// Use empty functions for panic handler.
// Panic handler can be implemented, e.g. to blink led or print message
// to serial, but I don't want it there.
use avr_std_stub as _;

// AVR HAL library. Crate is named "arduino_uno" in Cargo.toml.
use arduino_uno::prelude::*;

// Main function
#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    // Pin D13 is connected to L led
    let mut led = pins.d13.into_output(&mut pins.ddr);

    loop {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(500);
    }
}
