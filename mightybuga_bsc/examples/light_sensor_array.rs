// Use the line sensor to get a map with the light intensity of the sensor array.
//
// Used Hardware:
// Pin b1 for driving the led of the sensor array high
// Pins a0-7 for reading the values of the sensor array
// ADC1 for reading the voltages of the analog pins
// UART for logging the values to the terminal

#![no_std]
#![cfg_attr(not(doc), no_main)]

use light_sensor_array_controller::LightSensorArrayController;
use mightybuga_bsc as board;
use mightybuga_bsc::prelude::*;

use nb::block;

#[entry]
fn main() -> ! {
    let board = board::Mightybuga_BSC::take().unwrap();
    let mut delay = board.delay;
    let mut light_sensor_array = board.light_sensor_array;
    let uart = board.serial;

    let mut tx = uart.tx;

    light_sensor_array.set_led(true);

    loop {
        delay.delay(2000.millis());

        let values = light_sensor_array.get_light_map();
        defmt::println!("Values: {}", values);

        // Print the values of the sensor array separated by new lines.
        values.iter().for_each(|value| {
            let s = [
                b'0' + (value / 1000) as u8,
                b'0' + ((value / 100) % 10) as u8,
                b'0' + ((value / 10) % 10) as u8,
                b'0' + (value % 10) as u8,
                b' ',
                b' ',
            ];

            s.iter().for_each(|c| {
                block!(tx.write(*c)).unwrap();
            });
        });

        // Print a new line to separate the values of the sensor array.
        block!(tx.write(b'\r')).ok();
        block!(tx.write(b'\n')).ok();
    }
}
