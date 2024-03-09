// Use the line sensor to get a map with the light intensity of the sensor array.
//
// Used Hardware:
// Pin b1 for driving the led of the sensor array high
// Pins a0-7 for reading the values of the sensor array
// ADC1 for reading the voltages of the analog pins
// UART for logging the values to the terminal

#![no_std]
#![cfg_attr(not(doc), no_main)]

use mightybuga_bsc::prelude::*;
use panic_halt as _;
use mightybuga_bsc as board;

use nb::block;

#[entry]
fn main() -> ! {
    let board = board::Mightybuga_BSC::take().unwrap();
    let mut delay = board.delay;
    let mut line_sensor = board.line_sensor;
    let mut uart = board.uart;

    let mut tx = uart.tx;

    loop {
        delay.delay(2000.millis());

        let values = line_sensor.get_line_map();
        
        // Print the values of the sensor array separated by new lines.
        values.iter().for_each(|value| {
            let s = [b'0' + (value / 1000) as u8,
                     b'0' + ((value / 100) % 10) as u8,
                     b'0' + ((value / 10) % 10) as u8,
                     b'0' + (value % 10) as u8,
                     b'\r',
                     b'\n'];

            let _ = s.iter().map(|c| block!(tx.write(*c))).last();
        });

        // Separator to prevent confusions when reading the values in the terminal
        block!(tx.write('-'));
    }
}

