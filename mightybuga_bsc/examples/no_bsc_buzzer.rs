// Example of use of the PWD peripheral with the stm32f1xx_hal crate
//
// We will use the PWD peripheral to generate a PWM signal on pin PB04.
// PA4 is connected to a buzzer on the mightybuga_bsc board.
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

use panic_probe as _;
use defmt_rtt as _; // global logger

use stm32f1xx_hal::{pac, prelude::*, serial::*, timer::Channel};

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
    let (mut tx, mut rx) = serial.split();

    /* Print a nice hello message */
    let s = b"\r\nWellcome to the No BSC buzzer example:\r\n";

    let _ = s.iter().map(|c| block!(tx.write(*c))).last();

    // Acquire the GPIOB peripheral
    let mut gpiob = dp.GPIOB.split();

    let (_pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    // Configure gpio B pin 4 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let buzzer = pb4.into_alternate_push_pull(&mut gpiob.crl);
    let p1 = gpiob.pb5.into_alternate_push_pull(&mut gpiob.crl);

    let pins = (buzzer, p1); // I need to declare more than one here, why?

    // Configure the PWD peripheral
    let mut pwm = dp.TIM3.pwm_hz(pins, &mut afio.mapr, 1.kHz(), &clocks);

    pwm.enable(Channel::C1);

    let max = pwm.get_max_duty();
    pwm.set_duty(Channel::C1, max / 2);

    // set the frequency for the middle octave A note (440Hz)
    let middle_a = 440.Hz();
    // set the frequency for the middle octave B note (493.88Hz)
    let middle_b = 494.Hz();
    // set the frequency for the middle octave C note (523.25Hz)
    let middle_c = 523.Hz();
    // set the frequency for the middle octave D note (587.33Hz)
    let middle_d = 587.Hz();
    // set the frequency for the middle octave E note (659.25Hz)
    let middle_e = 659.Hz();
    // set the frequency for the middle octave F note (698.46Hz)
    let middle_f = 698.Hz();
    // set the frequency for the middle octave G note (783.99Hz)
    let middle_g = 784.Hz();

    // make a vector with the notes and durations of the song
    let song = [
        (middle_a, 1_u32),
        (middle_b, 1_u32),
        (middle_c, 1_u32),
        (middle_d, 1_u32),
        (middle_e, 1_u32),
        (middle_f, 1_u32),
        (middle_g, 1_u32),
    ];

    // loop over the notes of the song
    for _ in 0..2 {
        for (note, duration) in song.iter() {
            // set the frequency of the PWM signal
            pwm.set_period(*note);
            // wait for the duration of the note
            delay.delay_ms(*duration * 500_u32);

            let s = b"note played\r\n";
            let _ = s.iter().map(|c| block!(tx.write(*c))).last();
            led.toggle();
        }
    }

    pwm.set_duty(Channel::C1, 0);

    loop {
        delay.delay_ms(100_u16);
    }
}
