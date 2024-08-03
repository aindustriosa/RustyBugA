use crate::LineFollowerStatus;
use logging::Logger;

use crate::fsm_states;

pub enum FSMState {
    Idle,
    HardwareCheck,
    Calibration,
    LineFollowing,
    BatteryLow,
}

pub trait Runnable {
    fn run(&self, status: LineFollowerStatus) -> (FSMEvent, LineFollowerStatus);
}

#[derive(Clone, Copy)]
pub enum FSMEvent {
    NothingHappend,
    Button1Pressed,
    Button2Pressed,
    BatteryIsLow,
}

impl FSMState {
    pub fn next(
        self,
        event: FSMEvent,
        mut status: LineFollowerStatus,
    ) -> (FSMState, LineFollowerStatus) {
        match (self, event) {
            (FSMState::Idle, FSMEvent::Button1Pressed) => (FSMState::HardwareCheck, status),
            (FSMState::Idle, FSMEvent::Button2Pressed) => (FSMState::Calibration, status),
            (FSMState::Idle, FSMEvent::BatteryIsLow) => (FSMState::BatteryLow, status),

            (FSMState::HardwareCheck, FSMEvent::NothingHappend) => (FSMState::Idle, status),
            (FSMState::HardwareCheck, FSMEvent::BatteryIsLow) => (FSMState::BatteryLow, status),

            (FSMState::Calibration, FSMEvent::Button1Pressed) => (FSMState::LineFollowing, status),
            (FSMState::Calibration, FSMEvent::Button2Pressed) => (FSMState::Idle, status),
            (FSMState::Calibration, FSMEvent::BatteryIsLow) => (FSMState::BatteryLow, status),

            (FSMState::LineFollowing, FSMEvent::Button2Pressed) => (FSMState::Idle, status),
            (FSMState::LineFollowing, FSMEvent::BatteryIsLow) => (FSMState::BatteryLow, status),

            (_s, _e) => {
                Logger::new(&mut status.board.serial.tx).log("default to idle state\r\n");
                (FSMState::Idle, status)
            }
        }
    }

    pub fn run<'a>(&self, mut status: LineFollowerStatus) -> (FSMEvent, LineFollowerStatus) {
        let mut logger = Logger::new(&mut status.board.serial.tx);
        match *self {
            FSMState::Idle {} => fsm_states::idle::run(status),
            FSMState::HardwareCheck {} => fsm_states::hardware_check::run(status),
            FSMState::Calibration {} => fsm_states::calibration::run(status),
            FSMState::LineFollowing {} => fsm_states::line_following::run(status),
            FSMState::BatteryLow {} => fsm_states::battery_low::run(status),
        }
    }
}
