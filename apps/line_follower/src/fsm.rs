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

#[derive(Clone, Copy)]
pub enum FSMEvent {
    NothingHappened,
    Button1Pressed,
    Button2Pressed,
    BatteryIsLow,
}

impl FSMState {
    pub fn next(
        self,
        event: FSMEvent,
        status: &mut LineFollowerStatus,
    ) -> FSMState {
        match (self, event) {
            (FSMState::Idle, FSMEvent::Button1Pressed) => FSMState::HardwareCheck,
            (FSMState::Idle, FSMEvent::Button2Pressed) => FSMState::Calibration,
            (FSMState::Idle, FSMEvent::BatteryIsLow) => FSMState::BatteryLow,

            (FSMState::HardwareCheck, FSMEvent::NothingHappened) => FSMState::Idle,
            (FSMState::HardwareCheck, FSMEvent::BatteryIsLow) => FSMState::BatteryLow,

            (FSMState::Calibration, FSMEvent::Button1Pressed) => FSMState::LineFollowing,
            (FSMState::Calibration, FSMEvent::Button2Pressed) => FSMState::Idle,
            (FSMState::Calibration, FSMEvent::BatteryIsLow) => FSMState::BatteryLow,

            (FSMState::LineFollowing, FSMEvent::Button2Pressed) => FSMState::Idle,
            (FSMState::LineFollowing, FSMEvent::BatteryIsLow) => FSMState::BatteryLow,

            (_s, _e) => {
                Logger::new(&mut status.board.serial.tx).log("default to idle state\r\n");
                FSMState::Idle
            }
        }
    }

    pub fn run<'a>(&self, status: &mut LineFollowerStatus) -> FSMEvent {
        match *self {
            FSMState::Idle {} => fsm_states::idle::run(status),
            FSMState::HardwareCheck {} => fsm_states::hardware_check::run(status),
            FSMState::Calibration {} => fsm_states::calibration::run(status),
            FSMState::LineFollowing {} => fsm_states::line_following::run(status),
            FSMState::BatteryLow {} => fsm_states::battery_low::run(status),
        }
    }
}
