// Test the Mightybuga_BSC board buttons.
//
// Used Hardware:
// - Buttons: BTN1, BTN2, BTN3
// - LEDs: D1, D2

#![no_std]
#![cfg_attr(not(doc), no_main)]

use mightybuga_bsc as board;
use mightybuga_bsc::prelude::*;
use panic_halt as _;

use hal_button::ButtonController;

#[entry]
fn main() -> ! {
    let board = board::Mightybuga_BSC::take().unwrap();
    let mut delay = board.delay;
    let mut led_d1 = board.led_d1;
    let mut led_d2 = board.led_d2;
    let mut btn_1 = board.btn_1;
    let mut btn_2 = board.btn_2;
    let mut btn_3 = board.btn_3;

    loop {
        // check if any button changes its state
        if btn_1.is_changed() || btn_2.is_changed() || btn_3.is_changed() {
            // bheck the state of the buttons and turn on the LEDs accordingly
            match (btn_1.is_pressed(), btn_2.is_pressed(), btn_3.is_pressed()) {
                (true, false, false) => {
                    // button 1 is pressed: turn on LED D1
                    led_d1.set_high();
                    led_d2.set_low();
                }
                (false, true, false) => {
                    // button 2 is pressed: turn on LED D2
                    led_d1.set_low();
                    led_d2.set_high();
                }
                (false, false, true) => {
                    // button 3 is pressed: turn on both LEDs
                    led_d1.set_high();
                    led_d2.set_high();
                }
                _ => {
                    // turn off both LEDs
                    led_d1.set_low();
                    led_d2.set_low();
                }
            }
        }

        // don't burn the CPU
        delay.delay(40.millis());
    }
}
