#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate panic_halt;
use arduino_leonardo::prelude::*;

// This pin will be used from the interrupt handler
use arduino_leonardo::hal::port;
static mut PIN: Option<port::portc::PC7<port::mode::Output>> = None;

#[arduino_leonardo::entry]
fn main() -> ! {
    let dp = arduino_leonardo::Peripherals::take().unwrap();

    let mut pins = arduino_leonardo::Pins::new(
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
        dp.PORTE,
    );

    let mut led0 = pins.led_rx.into_output(&mut pins.ddr);
    let mut led1 = pins.led_tx.into_output(&mut pins.ddr);

    let mut led = pins.d13.into_output(&mut pins.ddr);

    led0.set_high().void_unwrap();
    led1.set_low().void_unwrap();
    led.set_low().void_unwrap();

    unsafe {
        PIN = Some(led);
    }

    // In theory this should not be necessary ... But if you previously had
    // a sketch from Arduino loaded, the USB device will not have been reset.
    // Because of this we will be spammed with interrupts which will never
    // stop because they are never handled.
    dp.USB_DEVICE.usbcon.reset();

    // Initialize INT6
    // There is not yet a hal implementation, which is why we need to do this
    // manually
    let ei = dp.EXINT;
    // TODO: Patch EXINT so we at least don't need manual values here
    ei.eicrb.write(|w| w.isc6().bits(0x02));
    ei.eimsk.write(|w| w.int().bits(0x40));

    // Enable interrupts
    unsafe {
        avr_device::interrupt::enable();
    }

    loop {
        led0.toggle().void_unwrap();
        led1.toggle().void_unwrap();
        arduino_leonardo::delay_ms(300);
    }
}

#[avr_device::interrupt(atmega32u4)]
unsafe fn INT6() {
    PIN.as_mut().unwrap().toggle().void_unwrap();
}
