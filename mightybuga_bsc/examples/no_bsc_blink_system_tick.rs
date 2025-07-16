//! Blinks an LED as a GPIO when the system has reached a number of milliseconds.
//! Milliseconds are counted using the system tick timer (SYST) of the Cortex-M core.
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
use defmt_rtt as _;
use panic_probe as _; // global logger

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use stm32f1xx_hal::{pac, prelude::*};

static mut MILLIS_COUNTER: u32 = 0;

#[exception]
fn SysTick() {
    // Increment the milliseconds counter every time the SysTick interrupt is triggered
    unsafe {
        MILLIS_COUNTER += 1;
    }
    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, ".").unwrap();
}

#[entry]
fn main() -> ! {
    let mut hstdout = hio::hstdout().unwrap();
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();

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
    //let mut delay = cp.SYST.delay(&clocks);

    let mut syst = cp.SYST;
    // Configure the system timer to trigger an interrupt every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // Assuming 8 MHz core clock
    syst.enable_counter();
    syst.enable_interrupt();

    loop {
        //delay.delay(300.millis());
        led.set_high();

        let millis_counter = unsafe { MILLIS_COUNTER };
        writeln!(hstdout, "Counter {}", millis_counter).unwrap();

        //delay.delay_ms(1_000_u16);
        led.set_low();

        let millis_counter = unsafe { MILLIS_COUNTER };
        writeln!(hstdout, "Counter {}", millis_counter).unwrap();
    }
}
