// app_tests_menu hardware independent application as a library
//
// application that presents over the serial port a menu with hardware
// related tests that can be run on the robot.
// 
#![no_std]

use mightybuga_bsc as board;
use board::prelude::*;
use board::timer::SysDelay;
use board::timer_based_buzzer::TimerBasedBuzzer;
use board::timer_based_buzzer::TimerBasedBuzzerInterface;
use board::EncoderController;
use board::defmt;

use battery_sensor_controller::BatterySensorController;
use engine::engine::EngineController;
use light_sensor_array_controller::LightSensorArrayController;

use nb::block;

extern crate alloc;

use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use logging::Logger;


pub struct AppTestsMenu;
impl AppTestsMenu {
    pub fn new() -> Self {
        AppTestsMenu
    }

    pub fn loop(
        &self,
        board: &mut board::Mightybuga_BSC,
    ) {
        let mut delay = board.delay;
        let mut uart = board.serial;
        let mut led_d1 = board.led_d1;
        let mut led_d2 = board.led_d2;
        let mut buzzer = board.buzzer;
        let mut engine = board.engine;
        let mut light_sensor_array = board.light_sensor_array;
        let mut battery_sensor = board.battery_sensor;

        let mut logger = Logger::new(&mut uart.tx);

        // Print the menu
        print_menu(&mut logger);

        // Read the user input
        if let Ok(byte) = block!(uart.rx.read()) {
            defmt::println!("Received byte: {}", byte);
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
                    print_number(
                        battery_sensor.get_battery_millivolts() as isize,
                        &mut logger,
                    );
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