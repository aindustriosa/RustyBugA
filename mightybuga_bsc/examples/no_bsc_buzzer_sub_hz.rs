// Example of use of the PWD peripheral with the stm32f1xx_hal crate
//
// We will use the PWD peripheral to generate a PWM signal on pin PB04.
// PA4 is connected to a buzzer on the mightybuga_bsc board.
//
// If we use pwm_hz we can only set the frequency to a minimum of 1Hz.
// We need to use pwm to set the frequency to a minimum of 0.01Hz.
//
// Used Hardware:
// The board exposes a single LED on pin PC13 and a UART on pins PA9 and PA10.
// The board exposes a buzzer on bluepill's pin PB04.
//
// References:
// https://docs.rs/crate/stm32f1xx-hal/0.10.0/source/examples/pwm_input.rs
// https://github.com/stm32-rs/stm32f1xx-hal/issues/77
// https://github.com/stm32-rs/stm32f1xx-hal/blob/master/examples/nojtag.rs

#![no_std]
#![cfg_attr(not(doc), no_main)]

use panic_halt as _;

use stm32f1xx_hal::{
    pac::{self},
    prelude::*,
    serial::*,
    timer::Channel,
};

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

    // 1. Enable the peripheral clock in the RCC register
    dp.RCC.apb1enr.write(|w| w.tim3en().set_bit());

    let rcc = dp.RCC.constrain();

    let mut afio = dp.AFIO.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .freeze(&mut flash.acr);
    // PAC level configuration at https://github.com/apollolabsdev/stm32-nucleo-f401re/blob/main/PAC%20Examples/sys_clocks/src/main.rs

    //let clocks = rcc.cfgr.freeze(&mut flash.acr);

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
    let (mut tx, mut rx) = serial.split();

    /* Print a nice hello message */
    let s = b"\r\nWellcome to the No BSC buzzer example:\r\n";

    let _ = s.iter().map(|c| block!(tx.write(*c))).last();

    // Acquire the GPIOB peripheral
    let mut gpiob = dp.GPIOB.split();

    let (_pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    // Configure gpio B pin 4 as a push-pull output. The `crl` register is passed to the function
    // in order to configure the port. For pins 8-15, crh should be passed instead.
    let buzzer = pb4.into_alternate_push_pull(&mut gpiob.crl);

    // Configure the PWD peripheral
    let psc = 70;
    let arr = 2903;

    // Configure the PWD peripheral at PAC level:

    // Set timer 3 mode to no divisor (72MHz), Edge-aligned, up-counting, 
    // enable Auto-Reload Buffering, continous mode, disable timer.
    dp.TIM3.cr1.write(|w| w.arpe().set_bit().cms().bits(0b00).dir().clear_bit().opm().clear_bit().cen().clear_bit());
    // set timer 3 prescaler to 70
    dp.TIM3.psc.write(|w| w.psc().bits(psc));
    // set timer 3 auto-reload register to 2903
    dp.TIM3.arr.write(|w| w.arr().bits(arr));
    // Timer Set Output Compare Mode to PWM Mode 1
    dp.TIM3.ccmr1_output().write(|w| w.oc1m().pwm_mode1());
    // enable the channel from TIMx capture/compare enable register
    dp.TIM3.ccer.write(|w| w.cc1e().set_bit());
    // Set duty cycle to 50% for channel 1
    dp.TIM3.ccr1().write(|w| w.ccr().bits(u16::MAX / 2));

    // Enable timer (note that write resets the not set bits, so we need to use modify here)
    dp.TIM3.cr1.modify(|_, w| w.cen().set_bit());

    loop {
        delay.delay_ms(1000_u16);
        // print hello
        let s = b"Hello, world!\r\n";
        let _ = s.iter().map(|c| block!(tx.write(*c))).last();
        // toggle led
        led.set_high();
        delay.delay_ms(1000_u16);
        led.set_low();
    }
}
