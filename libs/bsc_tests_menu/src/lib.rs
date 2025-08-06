// app_tests_menu hardware independent application as a library
//
// application that presents over the serial port a menu with hardware
// related tests that can be run on the robot.
// 
#![no_std]

use mightybuga_bsc_interface::{
    MightybugaBsc,
    BuzzerTrait,
    EngineTrait,
    LightSensorTrait,
    BatterySensorTrait,
    EncoderTrait,
    Serial,
    DelayMs,
    OutputPin,
    Read,
    Write,
    };

use nb::block;
extern crate alloc;
use core::convert::Infallible;

use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use logging::log;

pub struct BSCTestsMenu;

impl BSCTestsMenu {
    pub fn new() -> Self {
        BSCTestsMenu
    }

    /// Runs the main loop of the BSC tests menu.
    pub fn do_loop<B>(&self, board: &mut B) 
    where
        B: MightybugaBsc,
        B::LedD1: OutputPin,
        B::LedD2: OutputPin,
        B::Serial: Serial,
        B::Delay: DelayMs<u32>,
        B::Buzzer: BuzzerTrait,
        B::Engine: EngineTrait,
        B::LightSensorArray: LightSensorTrait,
        B::BatterySensor: BatterySensorTrait,
        B::EncoderL: EncoderTrait,
        B::EncoderR: EncoderTrait,
    {
        print_menu(board.serial().tx());

        // Read the user input
        if let Ok(byte) = block!(board.serial().rx().read()) {
            // defmt is embedded-specific, replace with logging
            log(board.serial().tx(), "Received byte: ");
            
            // Process the user input
            match byte {
                b'1' => {
                    // Play some notes with the buzzer
                    play_notes(board);
                }
                b'2' => {
                    // Turn on the LED D1
                    board.led_d1().set_high().ok();
                }
                b'3' => {
                    // Turn off the LED D1
                    board.led_d1().set_low().ok();
                }
                b'4' => {
                    // Turn on the LED D2
                    board.led_d2().set_high().ok();
                }
                b'5' => {
                    // Turn off the LED D2
                    board.led_d2().set_low().ok();
                }
                b'6' => {
                    // Read the left encoder
                    read_left_encoder(board);
                }
                b'7' => {
                    // Read the right encoder
                    read_right_encoder(board);
                }
                b'a' => {
                    // Move the robot forward
                    board.engine().move_robot(1000, 1000).ok();
                    board.delay().delay_ms(1000u32);
                    board.engine().stop().ok();
                }
                b's' => {
                    // Move the robot backward
                    board.engine().move_robot(-1000, -1000).ok();
                    board.delay().delay_ms(1000u32);
                    board.engine().stop().ok();
                }
                b'd' => {
                    // Turn the robot right
                    board.engine().move_robot(500, -500).ok();
                    board.delay().delay_ms(1000u32);
                    board.engine().stop().ok();
                }
                b'f' => {
                    // Turn the robot left
                    board.engine().move_robot(-500, 500).ok();
                    board.delay().delay_ms(1000u32);
                    board.engine().stop().ok();
                }
                b'g' => {
                    // Read the light sensors
                    log(board.serial().tx(), "Light sensor values: ");
                    if let Ok(light_values) = board.light_sensor_array().read_sensors() {
                        for value in &light_values {
                            log(board.serial().tx(), " ");
                            print_number(*value as i32, board.serial().tx());
                        }
                    }
                    log(board.serial().tx(), "\r\n");
                }
                b'h' => {
                    // Read the battery sensor
                    log(board.serial().tx(), "Battery sensor value: ");
                    if let Ok(voltage) = board.battery_sensor().read_voltage() {
                        print_number(voltage as i32, board.serial().tx());
                        log(board.serial().tx(), " mV");
                    }
                    log(board.serial().tx(), "\r\n");
                }
                _ => {
                    print_menu(board.serial().tx());
                }
            }
        }
        
        // Wait for a while
        board.delay().delay_ms(300u32);
        // print a dot to the user
        log(board.serial().tx(), ".\r\n");
    }
}

// Helper functions - you'll need to implement these based on your specific needs
fn print_menu<W>(tx: &mut W) 
where
    W: Write<u8, Error = Infallible>
{
    log(tx, "=== MightyBuga Test Menu ===\r\n");
    log(tx, "1. Play buzzer notes\r\n");
    log(tx, "2. LED D1 ON\r\n");
    log(tx, "3. LED D1 OFF\r\n");
    log(tx, "4. LED D2 ON\r\n");
    log(tx, "5. LED D2 OFF\r\n");
    log(tx, "6. Read left encoder\r\n");
    log(tx, "7. Read right encoder\r\n");
    log(tx, "a. Move forward\r\n");
    log(tx, "s. Move backward\r\n");
    log(tx, "d. Turn right\r\n");
    log(tx, "f. Turn left\r\n");
    log(tx, "g. Read light sensors\r\n");
    log(tx, "h. Read battery\r\n");
}

fn play_notes<B>(board: &mut B)
where
    B: MightybugaBsc,
    B::Buzzer: BuzzerTrait,
    B::Delay: DelayMs<u32>,
{
    board.buzzer().beep(440, 200).ok(); // A note
    board.delay().delay_ms(250u32);
    board.buzzer().beep(523, 200).ok(); // C note
    board.delay().delay_ms(250u32);
    board.buzzer().beep(659, 200).ok(); // E note
}

fn log_encoder<W>(value: i32, tx: &mut W)
where
    W: Write<u8, Error = Infallible>,
{
    log(tx,"Encoder count: ");
    print_number(value, tx);
    log(tx,"\r\n");
}

fn read_left_encoder<B>(board: &mut B)
where
    B: MightybugaBsc,
    B::EncoderL: EncoderTrait,
    B::Serial: Serial,
{
    let count = board.encoder_l().count();
    log_encoder(count, board.serial().tx());
}

fn read_right_encoder<B>(board: &mut B)
where
    B: MightybugaBsc,
    B::EncoderR: EncoderTrait,
    B::Serial: Serial,
{
    let count = board.encoder_r().count();
    log_encoder(count, board.serial().tx());
}

fn print_number<W>(num: i32, tx: &mut W)
where 
    W: Write<u8, Error = Infallible>
{
    // Simple number to string conversion - you might want to use a proper formatting library
    let mut buffer = [0u8; 20];
    let mut pos = 0;
    let mut n = num.abs() as usize;
    
    if n == 0 {
        buffer[0] = b'0';
        pos = 1;
    } else {
        while n > 0 {
            buffer[pos] = (n % 10) as u8 + b'0';
            n /= 10;
            pos += 1;
        }
    }
    
    if num < 0 {
        buffer[pos] = b'-';
        pos += 1;
    }
    
    // Reverse the string
    for i in 0..pos/2 {
        buffer.swap(i, pos - 1 - i);
    }
    
    log(tx, core::str::from_utf8(&buffer[..pos]).unwrap_or("?"));
}