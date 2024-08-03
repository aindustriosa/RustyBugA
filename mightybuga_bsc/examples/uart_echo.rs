// Example of a serial echo program using the mightybuga_bsc crate
//
// This example echoes back characters received on the UART.
//
// Used Hardware:
// The board exposes a single LED on pin PC13 and a UART on pins PA9 and PA10.

#![no_std]
#![cfg_attr(not(doc), no_main)]

use mightybuga_bsc as board;
use mightybuga_bsc::prelude::*;
use nb::block;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = board::Mightybuga_BSC::take().unwrap();
    let mut delay = board.delay;
    let mut uart = board.serial;
    let mut led_d1 = board.led_d1;

    let s = b"\r\nPlease type characters to echo:\r\n";
    let _ = s.iter().map(|c| block!(uart.tx.write(*c))).last();

    loop {
        if let Ok(byte) = block!(uart.rx.read()) {
            let _ = block!(uart.tx.write(byte));
            led_d1.toggle();
            delay.delay_ms(100_u16);
        }
    }
}
