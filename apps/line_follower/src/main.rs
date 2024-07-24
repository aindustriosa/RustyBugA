// Line follower application.

#![no_std]
#![no_main]

use panic_halt as _;

use mightybuga_bsc::{self as board, Mightybuga_BSC};
use mightybuga_bsc::prelude::*;

extern crate alloc;

use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use logging::Logger;

//#[derive(PartialEq)]
enum FSMState {
    Idle {},
    HardwareCheck {},
    Calibration {},
    LineFollowing {},
    BatteryLow {},
}

#[derive(Clone, Copy)]
enum FSMEvent {
    NothingHappend,
    Button1Pressed,
    Button2Pressed,
    BatteryIsLow,
}

impl FSMState {
    fn next(
        self,
        event: FSMEvent,
        mut status: LineFollowerStatus,
    ) -> (FSMState, LineFollowerStatus) {
        match (self, event) {
            (FSMState::Idle {}, FSMEvent::Button1Pressed) => (FSMState::HardwareCheck {}, status),
            (FSMState::Idle {}, FSMEvent::Button2Pressed) => (FSMState::Calibration {}, status),
            (FSMState::Idle {}, FSMEvent::BatteryIsLow) => (FSMState::BatteryLow {}, status),
            (FSMState::HardwareCheck {}, FSMEvent::NothingHappend) => {
                (FSMState::HardwareCheck {}, status)
            }
            (FSMState::HardwareCheck {}, FSMEvent::BatteryIsLow) => {
                (FSMState::BatteryLow {}, status)
            }
            (FSMState::Calibration {}, FSMEvent::NothingHappend) => {
                (FSMState::Calibration {}, status)
            }
            (FSMState::Calibration {}, FSMEvent::Button1Pressed) => {
                (FSMState::LineFollowing {}, status)
            }
            (FSMState::Calibration {}, FSMEvent::BatteryIsLow) => (FSMState::BatteryLow {}, status),
            (FSMState::LineFollowing {}, FSMEvent::BatteryIsLow) => {
                (FSMState::BatteryLow {}, status)
            }
            (_s, _e) => {
                Logger::new(&mut status.board.serial.tx).log("default to idle state\r\n");
                (FSMState::Idle {}, status)
            }
        }
    }

    fn run<'a>(&self, mut status: LineFollowerStatus) -> (FSMEvent, LineFollowerStatus) {
        let mut logger = Logger::new(&mut status.board.serial.tx);
        match *self {
            FSMState::Idle {} => {
                logger.log("Idle state\r\n");
                status.board.led_d1.set_high();
                status.board.led_d2.set_low();
                status.board.delay.delay_ms(1000u32);
                (FSMEvent::Button1Pressed, status)
            }
            FSMState::HardwareCheck {} => {
                logger.log("Hardware check state\r\n");
                status.board.delay.delay_ms(1000u32);
                (FSMEvent::BatteryIsLow, status)
            }
            FSMState::Calibration {} => {
                logger.log("Calibration state\r\n");
                status.board.delay.delay_ms(1000u32);
                (FSMEvent::NothingHappend, status)
            }
            FSMState::LineFollowing {} => {
                logger.log("Line following state\r\n");
                status.board.delay.delay_ms(1000u32);
                (FSMEvent::NothingHappend, status)
            }
            FSMState::BatteryLow {} => {
                logger.log("Battery low state\r\n");
                status.board.delay.delay_ms(1000u32);
                (FSMEvent::NothingHappend, status)
            }
        }
    }
}

// Line follower state shared between the different states
struct LineFollowerStatus {
    board: board::Mightybuga_BSC,
}

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