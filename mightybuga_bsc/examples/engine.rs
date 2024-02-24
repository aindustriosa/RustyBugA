// Uses the motors to move the robot in different directions.
//
// Used Hardware:
// The board exposes a single LED on pin PC13.

#![no_std]
#![cfg_attr(not(doc), no_main)]

use mightybuga_bsc::prelude::*;
use panic_halt as _;
use mightybuga_bsc as board;

use engine::engine::EngineController;

#[entry]
fn main() -> ! {
    let board = board::Mightybuga_BSC::take().unwrap();
    let mut delay = board.delay;
    let mut led_d1 = board.leds.d1;
    let mut engine = board.engine;

    engine.forward(u16::MAX / 4);
    delay.delay(1000.millis());
    engine.backward(u16::MAX / 4);
    delay.delay(1000.millis());
    engine.left(u16::MAX / 5, u16::MAX / 10);
    delay.delay(1000.millis());
    engine.right(u16::MAX / 5, u16::MAX / 10);
    delay.delay(1000.millis());
    engine.stop();

    loop {
        delay.delay(200.millis());
        led_d1.set_high();
        delay.delay_ms(100_u16);
        led_d1.set_low();
    }
}
