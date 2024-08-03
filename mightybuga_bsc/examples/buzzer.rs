// Example of usage of the buzzer as defined in this BSC.
//
// Used Hardware:
// The board exposes a single LED on pin PC13.
// The board exposes a buzzer on pin PB4.

#![no_std]
#![cfg_attr(not(doc), no_main)]

use mightybuga_bsc as board;
use mightybuga_bsc::prelude::*;
use mightybuga_bsc::timer_based_buzzer::TimerBasedBuzzerInterface;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = board::Mightybuga_BSC::take().unwrap();
    let mut delay = board.delay;
    let mut led_d1 = board.led_d1;
    let mut buzzer = board.buzzer;

    // loop only three times to avoid annoying the user
    for _ in 0..3 {
        delay.delay(2000.millis());
        led_d1.set_high();
        buzzer.change_frequency(70, 3000);
        buzzer.turn_on();
        delay.delay_ms(2000_u16);
        led_d1.set_low();
        buzzer.turn_off();
    }

    loop {
        delay.delay(200.millis());
        led_d1.set_high();
        delay.delay_ms(200_u16);
        led_d1.set_low();
    }
}
