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
};

use cortex_m_rt::entry;
use nb::block;

// Note struct defines the prescaler and counter values for a given note
struct Note {
    prescaler: u16,
    counter: u16,
}

// Notes are defined as constants
// Here are the notes for the C major scale
const C: Note = Note {
    prescaler: 70,
    counter: 2052,
};
const D: Note = Note {
    prescaler: 70,
    counter: 1828,
};
const E: Note = Note {
    prescaler: 70,
    counter: 1629,
};
const F: Note = Note {
    prescaler: 70,
    counter: 1537,
};
const G: Note = Note {
    prescaler: 70,
    counter: 1369,
};
const A: Note = Note {
    prescaler: 70,
    counter: 1220,
};
const B: Note = Note {
    prescaler: 70,
    counter: 1087,
};
const C2: Note = Note {
    prescaler: 70,
    counter: 1026,
};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();

    // Enable the timer 3 clock in the RCC register (we net to do this before the constrain)
    dp.RCC.apb1enr.modify(|_, w| w.tim3en().set_bit());

    let rcc = dp.RCC.constrain();

    let mut afio = dp.AFIO.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .freeze(&mut flash.acr);

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
    // Remap TIM3
    afio.mapr
        .modify_mapr(|r, w| unsafe { w.tim3_remap().bits(0b10) });

    // Configure gpio B pin 4 as a push-pull output. The `crl` register is passed to the function
    // in order to configure the port. For pins 8-15, crh should be passed instead.
    let buzzer = pb4.into_alternate_push_pull(&mut gpiob.crl);

    // Configure the PWD peripheral at PAC level:

    // Set timer 3 mode to no divisor (72MHz), Edge-aligned, up-counting,
    // enable Auto-Reload Buffering, continous mode, disable timer.
    dp.TIM3.cr1.write(|w| {
        w.arpe()
            .set_bit()
            .cms()
            .bits(0b00)
            .dir()
            .clear_bit()
            .opm()
            .clear_bit()
            .cen()
            .clear_bit()
    });

    // Timer Set Output Compare Mode to PWM Mode 1
    dp.TIM3.ccmr1_output().write(|w| w.oc1m().pwm_mode1());
    // enable the channel from TIMx capture/compare enable register
    dp.TIM3.ccer.write(|w| w.cc1e().set_bit());

    // Set the prescaler for note C
    dp.TIM3.psc.write(|w| w.psc().bits(C.prescaler));
    // Set the auto-reload value for note C
    dp.TIM3.arr.write(|w| w.arr().bits(C.counter));
    // Set duty cycle to 50% for channel 1
    dp.TIM3.ccr1().write(|w| w.ccr().bits(C.counter / 2));

    // Enable timer (note that write resets the not set bits, so we need to use modify here)
    dp.TIM3.cr1.modify(|_, w| w.cen().set_bit());

    // make a array of notes
    let notes = &[C, D, E, F, G, A, B, C2];

    // loop over the notes
    for note in notes.iter() {
        // Set the prescaler for the note
        dp.TIM3.psc.write(|w| w.psc().bits(note.prescaler));
        // Set the auto-reload value for the note
        dp.TIM3.arr.write(|w| w.arr().bits(note.counter));
        // Set duty cycle to 50% for channel 1
        dp.TIM3.ccr1().write(|w| w.ccr().bits(note.counter / 2));
        // wait for 1 second
        delay.delay_ms(1000_u16);
    }

    // stop the timer
    dp.TIM3.cr1.modify(|_, w| w.cen().clear_bit());

    loop {
        delay.delay_ms(1000_u16);
        led.set_high();
        delay.delay_ms(1000_u16);
        led.set_low();
    }
}
