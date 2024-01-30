use embedded_hal::digital::v2::OutputPin;

#[derive(PartialEq, Debug)]
pub enum MotorState {
    Forward,
    Backward,
}

pub trait MotorController {
    fn set_state(&mut self, state: MotorState);
    fn set_speed(&mut self, speed: i16);

    fn forward(&mut self) {
        self.set_state(MotorState::Forward);
    }

    fn backward(&mut self) {
        self.set_state(MotorState::Backward);
    }

    fn stop(&mut self) {
        self.set_speed(0);
    }
}

// This is an struct to handle all the options regarding a motor.
pub struct Motor<A: OutputPin, B: OutputPin> {
    action_pin: A,
    direction_pin: B,
    // @TODO: I guess that motor has a PWM pin to set the speed
}

impl<A: OutputPin, B: OutputPin> Motor<A, B> {
    pub fn new(action_pin: A, direction_pin: B) -> Self {
        Motor {
            action_pin,
            direction_pin,
        }
    }
}

impl<A: OutputPin, B: OutputPin> MotorController for Motor<A, B> {
    fn set_state(&mut self, state: MotorState) {
        match state {
            MotorState::Backward => {
                let _ = self.action_pin.set_low();
                let _ = self.direction_pin.set_high();
            }
            MotorState::Forward => {
                let _ = self.action_pin.set_high();
                let _ = self.direction_pin.set_low();
            }
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
    fn test_motor_forward() {
        // given
        let mut action_pin = MockFakePin::new();
        action_pin.expect_set_low().times(0).returning(|| Ok(()));
        action_pin.expect_set_high().times(1).returning(|| Ok(()));

        let mut direction_pin = MockFakePin::new();
        direction_pin.expect_set_low().times(1).returning(|| Ok(()));
        direction_pin
            .expect_set_high()
            .times(0)
            .returning(|| Ok(()));

        // when
        let mut motor = Motor::new(action_pin, direction_pin);
        motor.forward();
    }

    #[test]
    fn test_motor_backward() {
        // given
        let mut action_pin = MockFakePin::new();
        action_pin.expect_set_low().times(1).returning(|| Ok(()));
        action_pin.expect_set_high().times(0).returning(|| Ok(()));

        let mut direction_pin = MockFakePin::new();
        direction_pin.expect_set_low().times(0).returning(|| Ok(()));
        direction_pin
            .expect_set_high()
            .times(1)
            .returning(|| Ok(()));

        // when
        let mut motor = Motor::new(action_pin, direction_pin);
        motor.backward();
    }
}
