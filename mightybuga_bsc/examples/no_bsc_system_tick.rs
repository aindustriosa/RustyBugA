//! based on https://github.com/stm32-rs/stm32f1xx-hal/blob/master/examples/blinky_timer_irq.rs
//! blinky timer using interrupts on TIM2
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Please note according to RM0008:
//! "Due to the fact that the switch only sinks a limited amount of current (3 mA), the use of
//! GPIOs PC13 to PC15 in output mode is restricted: the speed has to be limited to 2MHz with
//! a maximum load of 30pF and these IOs must not be used as a current source (e.g. to drive a LED)"

#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _; // global logger

use core::fmt::Write;
use cortex_m_semihosting::hio;

use stm32f1xx_hal as hal;

use crate::hal::{
    pac::{interrupt, Interrupt, Peripherals, TIM2},
    prelude::*,
    timer::{CounterUs, Event},
};

use core::cell::RefCell;
use cortex_m::{asm::wfi, interrupt::Mutex};
use cortex_m_rt::entry;

// Make timer interrupt registers globally available
static G_TIM: Mutex<RefCell<Option<CounterUs<TIM2>>>> = Mutex::new(RefCell::new(None));

// Should this be in a mutex?
// https://stackoverflow.com/questions/52784613/which-variable-types-sizes-are-atomic-on-stm32-microcontrollers
static mut MILLIS_COUNTER: u32 = 0;

// Define an interupt handler, i.e. function to call when interrupt occurs.
// This specific interrupt will "trip" when the timer TIM2 times out
#[interrupt]
fn TIM2() {
    static mut TIM: Option<CounterUs<TIM2>> = None;

    unsafe {
        MILLIS_COUNTER += 1;
    }

    let tim = TIM.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_TIM.borrow(cs).replace(None).unwrap())
    });

    let _ = tim.wait();
}

#[entry]
fn main() -> ! {
    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, "Hello, world!").unwrap();

    let dp = Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    // Use external crystal for clock
    // http://nercury.github.io/rust/embedded/experiments/2019/01/27/rust-embedded-02-measuring-the-clock.html#the-clock-problem
    let mut clocks = dp
        .RCC
        .constrain()
        .cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .freeze(&mut flash.acr);

    // Configure PC13 pin to blink LED
    let mut gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // Set up a timer expiring after 1s
    let mut timer = dp.TIM2.counter_us(&mut clocks);

    timer.start(1.millis()).unwrap();

    // Generate an interrupt when the timer expires
    timer.listen(Event::Update);

    // Move the timer into our global storage
    cortex_m::interrupt::free(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));
    
    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }

    loop {
        wfi();

        let millis_counter = unsafe { MILLIS_COUNTER };

        if millis_counter % 1000 == 1 {
            let _ = led.toggle();
            writeln!(hstdout, "Counter {}", millis_counter).unwrap();
        }
    }
}
