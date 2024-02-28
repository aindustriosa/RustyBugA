use embedded_hal::digital::v2::OutputPin;
use embedded_hal::PwmPin;

#[derive(PartialEq, Debug)]
pub enum MotorState {
    Forward,
    Backward,
    Brake,
}

pub trait MotorController {
    fn set_state(&mut self, state: MotorState);

    // duty goes from 0 to 65535
    fn set_duty(&mut self, duty: u16);

    fn forward(&mut self) {
        self.set_state(MotorState::Forward);
    }

    fn backward(&mut self) {
        self.set_state(MotorState::Backward);
    }

    fn stop(&mut self) {
        self.set_duty(0);
    }
}

// This is an struct to handle all the options regarding a motor.
pub struct Motor<A: OutputPin, B: OutputPin, P: PwmPin<Duty = u16>> {
    in_1: A,
    in_2: B,
    pwm: P,
}

impl<A: OutputPin, B: OutputPin, P: PwmPin<Duty = u16>> Motor<A, B, P> {
    pub fn new(in_1: A, in_2: B, pwm: P) -> Self {
        Motor { in_1, in_2, pwm }
    }
}

impl<A: OutputPin, B: OutputPin, P: PwmPin<Duty = u16>> MotorController for Motor<A, B, P> {
    // Given that the motor driver is a TB6612FNG (https://www.pololu.com/file/0J86/TB6612FNG.pdf),
    // the following logic is used to control the motor:
    //
    // 1. Forward: in_1 = 0, in_2 = 1
    // 2. Backward: in_1 = 1, in_2 = 0
    // 3. Brake: in_1 = 1, in_2 = 1  # this creates a short circuit, it is not recommended to use it for a long time
    // 4. Stop: speed = 0
    fn set_state(&mut self, state: MotorState) {
        match state {
            MotorState::Backward => {
                let _ = self.in_1.set_high();
                let _ = self.in_2.set_low();
            }
            MotorState::Forward => {
                let _ = self.in_1.set_low();
                let _ = self.in_2.set_high();
            }
            MotorState::Brake => {
                let _ = self.in_1.set_high();
                let _ = self.in_2.set_high();
            }
        };
    }

    fn set_duty(&mut self, duty: u16) {
        // We need to map the duty cycle from 0-65535 to 0-max_duty without loosing precision
        let max_duty = self.pwm.get_max_duty();
        let duty = (duty as u32 * max_duty as u32 / 65535) as u16;
        self.pwm.set_duty(duty);
    }
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

    mock! {
      FakePwmPin {}

      impl PwmPin for FakePwmPin {
        type Duty = u16;

        // Required methods
        fn enable(&mut self);
        fn disable(&mut self);
        fn get_duty(&self) -> u16;
        fn get_max_duty(&self) -> u16;
        fn set_duty(&mut self, duty: u16);
      }
    }

    #[test]
    fn test_motor_forward() {
        // given
        let mut in_1 = MockFakePin::new();
        in_1.expect_set_low().times(1).returning(|| Ok(()));
        in_1.expect_set_high().times(0).returning(|| Ok(()));

        let mut in_2 = MockFakePin::new();
        in_2.expect_set_low().times(0).returning(|| Ok(()));
        in_2
            .expect_set_high()
            .times(1)
            .returning(|| Ok(()));

        let pwm_pin = MockFakePwmPin::new();

        // when
        let mut motor = Motor::new(in_1, in_2, pwm_pin);
        motor.forward();
    }

    #[test]
    fn test_motor_backward() {
        // given
        let mut in_1 = MockFakePin::new();
        in_1.expect_set_low().times(0).returning(|| Ok(()));
        in_1.expect_set_high().times(1).returning(|| Ok(()));

        let mut in_2 = MockFakePin::new();
        in_2.expect_set_low().times(1).returning(|| Ok(()));
        in_2
            .expect_set_high()
            .times(0)
            .returning(|| Ok(()));

        let pwm_pin = MockFakePwmPin::new();

        // when
        let mut motor = Motor::new(in_1, in_2, pwm_pin);
        motor.backward();
    }
}
