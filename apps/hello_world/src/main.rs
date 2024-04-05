// Application that uses the mightybyga_bsc crate and libs to demonstrate the RustyBugA board capabilities.

#![no_std]
#![no_main]

use panic_halt as _;

use mightybuga_bsc as board;
use mightybuga_bsc::prelude::*;
use mightybuga_bsc::timer::SysDelay;
use mightybuga_bsc::timer_based_buzzer::TimerBasedBuzzer;
use mightybuga_bsc::timer_based_buzzer::TimerBasedBuzzerInterface;

use engine::engine::EngineController;

use nb::block;

extern crate alloc;

use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use logging::Logger;

#[entry]
fn main() -> ! {
    let board = board::Mightybuga_BSC::take().unwrap();
    let mut delay = board.delay;
    let mut uart = board.serial;
    let mut led_d1 = board.led_d1;
    let mut led_d2 = board.led_d2;
    let mut buzzer = board.buzzer;
    let mut engine = board.engine;

    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

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
    logger.log("   a. Move the robot forward\r\n");
    logger.log("   s. Move the robot backward\r\n");
    logger.log("   d. Turn the robot right\r\n");
    logger.log("   f. Turn the robot left\r\n");
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
