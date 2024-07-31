//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![no_std]
#![cfg_attr(not(doc), no_main)]

use core::fmt::Write;
// Note that Semihosting needs somebody listening (STLink). If you program this code with
// semihosting and reset it without an OpenOCD listening the program will halt on the first
// writeln.
// https://wiki.segger.com/Semihosting
use cortex_m_semihosting::hio;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, "Hello, world!").unwrap();

    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split();

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    //let mut delay = hal::timer::Timer::syst(cp.SYST, &clocks).delay();
    // or
    let mut delay = cp.SYST.delay(&clocks);

    // Wait for the timer to trigger an update and change the state of the LED
    let mut i = 0;
    loop {
        delay.delay(1.secs());
        led.set_high();
        delay.delay_ms(1_000_u16);
        led.set_low();
        i += 1;
        writeln!(hstdout, "Hello again; I have blinked {} times.", i).unwrap();
        if i == 10 {
            panic!("Yow, 10 times is enough!");
        }
    }
}
