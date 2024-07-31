//! battery sensor example without using the BSC
//! The battery sensor is a voltage divider with a 47k resistor and a 20k resistor.
//! The voltage is read from the middle of the two resistors.
//! The voltage is then converted to millivolts and printed to the serial port.
//!
//! Hardware connections:
//! - VBATT to the 47k resistor. The battery has 2 cells in series, so the maximum voltage is 8.4V
//! - GND to the 20k resistor
//! - The middle of the two resistors to PB0 (ADC1)
//! - Led to PC13
//! - Serial port to PA9 (tx) and PA10 (rx)
//!
//! Run with:
//! cargo xtask mightybuga_bsc example no_bsc_batt_sensor
//! or
//! cd mightybuga-bsc; cargo run --example no_bsc_batt_sensor

#![no_std]
#![cfg_attr(not(doc), no_main)]
use panic_halt as _;

use mightybuga_bsc as board;

use board::hal::adc::*;
use board::hal::serial::*;
use board::hal::{pac, prelude::*};
use cortex_m_rt::entry;
use nb::block;

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

    let mut afio = dp.AFIO.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // create a delay abstraction based on SysTick
    let mut delay = cp.SYST.delay(&clocks);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split();

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // turn on the led
    led.set_high();

    // Acquire the GPIOA peripheral
    let mut gpioa = dp.GPIOA.split();

    // Configure gpio A pins 9 and 10 as a push-pull output. The `crh` register is passed to the
    // function in order to configure the port. For pins 0-7, crl should be passed instead.
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    // Configure the serial peripheral
    let serial = Serial::new(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        Config::default()
            .baudrate(115_200.bps())
            .wordlength_8bits()
            .parity_none(),
        &clocks,
    );

    // Split the serial struct into a receiving and a transmitting part
    let (mut tx, _rx) = serial.split();

    let s = b"\r\nBattery readings:\r\n";
    let _ = s.iter().map(|c| block!(tx.write(*c))).last();

    // Acquire the ADC1 peripheral
    let mut adc1 = Adc::adc1(dp.ADC1, clocks);

    // Get the GPIOB port
    let mut gpiob = dp.GPIOB.split();

    // Configure PB0 as an analog input
    let mut pb0 = gpiob.pb0.into_analog(&mut gpiob.crl);

    loop {
        // Read the battery voltage, take 100 samples and average them (later, so we don't lose precision):
        let mut battery_voltage_by_100: u32 = 0;
        for _ in 0..100 {
            let sample: u16 = adc1.read(&mut pb0).unwrap();
            battery_voltage_by_100 += sample as u32;
        }

        // Convert the voltage to millivolts. We multiply the raw value by the battery voltage and
        // divide by the resister divider. The maximum value is 8.4V and the resister divider is
        // 47k and 20k. The raw value is 12 bits, so the maximum value is 4096 that corresponds to
        // 2.5 volts in the ADC. The formula is:
        let raw_to_millivolts_multiplier_by_1000 = 2857;
        let battery_voltage_mv =
            (battery_voltage_by_100 * raw_to_millivolts_multiplier_by_1000 / 100000) as u16;

        // Print the raw value and the battery voltage to the serial port
        let mut buf = [0; 5];
        u16_to_str((battery_voltage_by_100 / 100) as u16, &mut buf);
        let _ = buf.iter().map(|c| block!(tx.write(*c))).last();

        let s = b" - ";
        let _ = s.iter().map(|c| block!(tx.write(*c))).last();

        let mut buf = [0; 5];
        u16_to_str(battery_voltage_mv as u16, &mut buf);
        let _ = buf.iter().map(|c| block!(tx.write(*c))).last();

        let s = b" mVolts\r\n";
        let _ = s.iter().map(|c| block!(tx.write(*c))).last();

        delay.delay_ms(500_u16);
        led.toggle();
    }
}

// function that fills a buffer with the string representation of a number
fn u16_to_str(n: u16, buf: &mut [u8; 5]) {
    let mut n = n;
    for i in (0..5).rev() {
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
}
