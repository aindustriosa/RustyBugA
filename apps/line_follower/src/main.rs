// Line follower application.

#![no_std]
#![no_main]

use panic_halt as _;

use mightybuga_bsc::prelude::*;
use mightybuga_bsc::{self as board, Mightybuga_BSC};

extern crate alloc;

use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

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
        (fsm_event, line_follower_status) = fsm_state.run(line_follower_status);
        log_event(&mut line_follower_status.board, fsm_event);
        (fsm_state, line_follower_status) = fsm_state.next(fsm_event, line_follower_status);
    }
}

fn log_event(board: &mut Mightybuga_BSC, event: FSMEvent) {
    let mut logger = Logger::new(&mut board.serial.tx);
    match event {
        FSMEvent::NothingHappend => logger.log("Nothing happened\r\n"),
        FSMEvent::Button1Pressed => logger.log("Button 1 pressed\r\n"),
        FSMEvent::Button2Pressed => logger.log("Button 2 pressed\r\n"),
        FSMEvent::BatteryIsLow => logger.log("Battery is low\r\n"),
    }
}
