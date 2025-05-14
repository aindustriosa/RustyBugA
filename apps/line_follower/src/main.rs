// Line follower application.

#![no_std]
#![no_main]

use panic_probe as _;

use mightybuga_bsc::prelude::*;
use mightybuga_bsc::{self as board, Mightybuga_BSC};
use mightybuga_bsc::defmt; // for logging through the semihosting interface

extern crate alloc;

use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

// This is not the defmt logger, but a simple one
// that uses the serial interface to log messages.
use logging::Logger;

mod fsm;
use fsm::{FSMEvent, FSMState};
mod fsm_states;

mod line_follower_status;
use line_follower_status::LineFollowerStatus;

#[entry]
fn main() -> ! {
    let board = board::Mightybuga_BSC::take().unwrap();

    let mut line_follower_status = LineFollowerStatus { board };

    let mut fsm_state = FSMState::Idle {};
    let mut fsm_event;

    loop {
        fsm_event = fsm_state.run(&mut line_follower_status);
        log_event(&mut line_follower_status.board, fsm_event);
        fsm_state = fsm_state.next(fsm_event, &mut line_follower_status);
    }
}

fn log_event(board: &mut Mightybuga_BSC, event: FSMEvent) {
    let mut logger = Logger::new(&mut board.serial.tx);
    match event {
        FSMEvent::NothingHappened => {
            logger.log(" - Nothing happened -\r\n");
            defmt::info!(" - Nothing happened -\r\n");
        }
        FSMEvent::Button1Pressed => {
            logger.log(" - Button 1 pressed -\r\n");
            defmt::info!(" - Button 1 pressed -\r\n");
        }
        FSMEvent::Button2Pressed => {
            logger.log(" - Button 2 pressed -\r\n");
            defmt::info!(" - Button 2 pressed -\r\n");
        }
        FSMEvent::BatteryIsLow => {
            logger.log(" - Battery is low -\r\n");
            defmt::warn!(" - Battery is low -\r\n");
        }
    }
}
