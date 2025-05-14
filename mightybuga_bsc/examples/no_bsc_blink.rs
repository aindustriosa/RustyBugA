//! Blinks the Blue Pill's LED

#![no_std]
#![cfg_attr(not(doc), no_main)]

use panic_probe as _;
use defmt_rtt as _; // global logger

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
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

    loop {
        delay.delay(300.millis());
        led.set_high();
        delay.delay_ms(2_000_u16);
        led.set_low();
    }
}
