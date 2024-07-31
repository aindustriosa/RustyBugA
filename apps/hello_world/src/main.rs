// Application that uses the mightybyga_bsc crate and libs to demonstrate the RustyBugA board capabilities.

#![no_std]
#![no_main]

use panic_halt as _;

use mightybuga_bsc as board;
use mightybuga_bsc::prelude::*;
use mightybuga_bsc::timer::SysDelay;
use mightybuga_bsc::timer_based_buzzer::TimerBasedBuzzer;
use mightybuga_bsc::timer_based_buzzer::TimerBasedBuzzerInterface;
use mightybuga_bsc::EncoderController;

use engine::engine::EngineController;
use light_sensor_array_controller::LightSensorArrayController;
use battery_sensor_controller::BatterySensorController;

use nb::block;

extern crate alloc;

use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use logging::Logger;

#[entry]
fn main() -> ! {
    let mut board = board::Mightybuga_BSC::take().unwrap();
    let mut delay = board.delay;
    let mut uart = board.serial;
    let mut led_d1 = board.led_d1;
    let mut led_d2 = board.led_d2;
    let mut buzzer = board.buzzer;
    let mut engine = board.engine;
    let mut light_sensor_array = board.light_sensor_array;
    let mut battery_sensor = board.battery_sensor;

    let mut logger = Logger::new(&mut uart.tx);

    loop {
        // Print the menu
        print_menu(&mut logger);

        // Read the user input
        if let Ok(byte) = block!(uart.rx.read()) {
            // Process the user input
            match byte {
                b'1' => {
                    // Play some notes with the buzzer
                    play_notes(&mut logger, &mut buzzer, &mut delay);
                }
                b'2' => {
                    // Turn on the LED D1
                    led_d1.set_high();
                }
                b'3' => {
                    // Turn off the LED D1
                    led_d1.set_low();
                }
                b'4' => {
                    // Turn on the LED D1
                    led_d2.set_high();
                }
                b'5' => {
                    // Turn off the LED D1
                    led_d2.set_low();
                }
                b'6' => {
                    // Read the left encoder
                    read_encoder(&mut board.encoder_l, &mut logger, &mut delay);
                }
                b'7' => {
                    // Read the right encoder
                    read_encoder(&mut board.encoder_r, &mut logger, &mut delay);
                }
                b'a' => {
                    // Move the robot forward
                    engine.forward(u16::MAX / 4);
                    delay.delay(1000.millis());
                    engine.stop();
                }
                b's' => {
                    // Move the robot backward
                    engine.backward(u16::MAX / 4);
                    delay.delay(1000.millis());
                    engine.stop();
                }
                b'd' => {
                    // Turn the robot right
                    engine.right(u16::MAX / 5, u16::MAX / 10);
                    delay.delay(1000.millis());
                    engine.stop();
                }
                b'f' => {
                    // Turn the robot left
                    engine.left(u16::MAX / 5, u16::MAX / 10);
                    delay.delay(1000.millis());
                    engine.stop();
                }
                b'g' => {
                    // Read the light sensors
                    light_sensor_array.set_led(true);
                    delay.delay(500.millis());
                    logger.log("Light sensor values: ");
                    let light_map = light_sensor_array.get_light_map();
                    light_sensor_array.set_led(false);
                    for i in 0..8 {
                        logger.log(" ");
                        print_number(light_map[i] as isize, &mut logger);
                    }
                    logger.log("\r\n");
                }
                b'h' => {
                    // Read the battery sensor
                    logger.log("Battery sensor value: ");
                    print_number(battery_sensor.get_battery_millivolts() as isize, &mut logger);
                    logger.log(" milli Volts\r\n");
                }
                _ => {
                    // Print the menu
                    print_menu(&mut logger);
                }
            }
        }
        // Wait for a while
        delay.delay(300.millis());
        // print a dot to the user
        logger.log(".\r\n");
    }
}

// function that prints a menu to the user
fn print_menu(logger: &mut Logger) {
    logger.log("Menu:\r\n");
    logger.log("   1. Play some notes with the buzzer\r\n");
    logger.log("   2. Turn on the LED D1\r\n");
    logger.log("   3. Turn off the LED D1\r\n");
    logger.log("   4. Turn on the LED D2\r\n");
    logger.log("   5. Turn off the LED D2\r\n");
    logger.log("   6. Read the left encoder\r\n");
    logger.log("   7. Read the right encoder\r\n");
    logger.log("   a. Move the robot forward\r\n");
    logger.log("   s. Move the robot backward\r\n");
    logger.log("   d. Turn the robot right\r\n");
    logger.log("   f. Turn the robot left\r\n");
    logger.log("   g. Read the light sensors\r\n");
    logger.log("   h. Read the battery sensor\r\n");
    logger.log("   Any other key prints this menu\r\n");
}

// Note struct defines the prescaler and counter values for a given note
struct Note {
    prescaler: u16,
    counter: u16,
}

// function that plays some notes with the buzzer
fn play_notes(logger: &mut Logger, buzzer: &mut TimerBasedBuzzer, delay: &mut SysDelay) {
    logger.log("Playing some notes with the buzzer\r\n");

    // Define the notes
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

    // Play the notes
    buzzer.turn_on();
    buzzer.change_frequency(C.prescaler, C.counter);
    delay.delay(300.millis());
    buzzer.change_frequency(D.prescaler, D.counter);
    delay.delay(300.millis());
    buzzer.change_frequency(E.prescaler, E.counter);
    delay.delay(300.millis());
    buzzer.turn_off();
}

fn read_encoder(encoder: &mut mightybuga_bsc::IncrementalEncoder, logger: &mut Logger, delay: &mut SysDelay) {
    encoder.enable();
    logger.log("move the encoder! (and reset MCU to exit)\r\n");

    let mut last = 0;
    loop {
        let (delta, steps) = encoder.delta();
        if last != steps{
            logger.log("(steps,delta): (");
            print_number(steps, logger);
            logger.log(",");
            print_number(delta, logger);
            logger.log(")\r\n");
            last = steps;
        }

        // don't burn the CPU
        delay.delay(20.millis());
    }
}

fn print_number(n: isize, logger: &mut Logger) {
    let mut len = 0;
        let mut digits = [0_u8; 20];
        let mut d = 0;
        let mut binary = n.abs();
        while binary > 0 {
            digits[d] = (binary % 10) as u8;
            d += 1;
            binary /= 10;
        }
        if d == 0 {
            d = 1;
        }
        let mut ascii = [0_u8; 20];
        if n < 0 {
            ascii[0] = b'-';
            len = 1;
        }
        while d > 0 {
            d -= 1;
            ascii[len] = digits[d] + b'0';
            len += 1;
        }
    let s = core::str::from_utf8(ascii[0..len].as_ref()).unwrap();
    logger.log(s);
}