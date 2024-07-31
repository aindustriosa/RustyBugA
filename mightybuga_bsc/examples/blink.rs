// Blinks the Blue Pill's LED
//
// Used Hardware:
// The board exposes a single LED on pin PC13.

#![no_std]
#![cfg_attr(not(doc), no_main)]

use mightybuga_bsc::prelude::*;
use panic_halt as _;
use mightybuga_bsc as board;

#[entry]
fn main() -> ! {
    let board = board::Mightybuga_BSC::take().unwrap();
    let mut delay = board.delay;
    let mut led_d1 = board.led_d1;
    let mut led_d2 = board.led_d2;

    loop {
        delay.delay(200.millis());
        led_d1.set_high();
        led_d2.set_low();
        delay.delay_ms(100_u16);
        led_d1.set_low();
        led_d2.set_high();
    }
}
