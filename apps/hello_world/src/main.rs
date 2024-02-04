// Application that uses the mightybyga_bsc crate and libs to demonstrate the RustyBugA board capabilities.

#![no_std]
#![no_main]

use board::timer::SysDelay;
use board::timer_based_buzzer::TimerBasedBuzzer;
use board::timer_based_buzzer::TimerBasedBuzzerInterface;
use mightybuga_bsc as board;
use mightybuga_bsc::prelude::*;
use panic_halt as _;

extern crate alloc;

use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use logging::Logger;

#[entry]
fn main() -> ! {
    let board = board::Mightybuga_BSC::take().unwrap();
    let mut delay = board.delay;
    let mut uart = board.uart;
    let mut led_d1 = board.leds.d1;
    let mut buzzer = board.buzzer;

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
        let input = uart.rx.read().unwrap();

        // Process the user input
        match input {
            b'1' => {
                // Play some notes with the buzzer
                play_notes(&mut logger, &mut board.buzzer, &mut delay);
            }
            _ => {
                // Print the menu
                print_menu(&mut logger);
            }
        }
    }
}

// function that prints a menu to the user
fn print_menu(logger: &mut Logger) {
    logger.log("Menu:\r\n");
    logger.log("   1. Play some notes with the buzzer\r\n");
    logger.log("   Any other key. Prints this menu\r\n");
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
    delay.delay_ms(500);
    buzzer.change_frequency(D.prescaler, D.counter);
    delay.delay_ms(500);
    buzzer.change_frequency(E.prescaler, E.counter);
    delay.delay_ms(500);
    buzzer.turn_off();
}
