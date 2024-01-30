use embedded_hal::digital::v2::OutputPin;

#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
      FakePin {}

      impl OutputPin for FakePin {
        type Error = u32;

        // Required methods
        fn set_low(&mut self) -> Result<(), u32>;
        fn set_high(&mut self) -> Result<(), u32>;
      }
    }

    #[test]
    fn test_motor_start() {
        // given
        let mut mock = MockFakePin::new();
        mock.expect_set_low().times(0).returning(|| Ok(()));
        mock.expect_set_high().times(1).returning(|| Ok(()));

        // when
        let mut motor = Motor::new(mock);
        motor.start();
    }

    #[test]
    fn test_motor_stop() {
        // given
        let mut mock = MockFakePin::new();
        mock.expect_set_low().times(1).returning(|| Ok(()));
        mock.expect_set_high().times(0).returning(|| Ok(()));

        // when
        let mut motor = Motor::new(mock);
        motor.stop();
    }
}
