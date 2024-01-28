use embedded_hal::digital::v2::OutputPin;

pub enum MotorState {
    Start,
    Stop,
}

pub trait MotorController {
    fn set_state(&mut self, state: MotorState);
    fn set_speed(&mut self, speed: i16);

    fn start(&mut self) {
        self.set_state(MotorState::Start);
    }

    fn stop(&mut self) {
        self.set_state(MotorState::Stop);
    }
}

// This is an struct to handle all the options regarding a motor.
pub struct Motor<T: OutputPin> {
    pin: T,
    // @TODO: I guess that motor has a PWM pin to set the speed
}

impl<T: OutputPin> Motor<T> {
    pub fn new(pin: T) -> Self {
        Motor { pin }
    }
}

impl<T: OutputPin> MotorController for Motor<T> {
    fn set_state(&mut self, state: MotorState) {
        let _ = match state {
            MotorState::Stop => self.pin.set_low(),
            MotorState::Start => self.pin.set_high(),
        };
    }

    fn set_speed(&mut self, _speed: i16) {}
}
