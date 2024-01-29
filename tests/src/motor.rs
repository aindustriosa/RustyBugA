use embedded_hal::digital::v2::OutputPin;
#[allow(unused_imports)]
use mightybuga_bsc::motor::Motor;
#[allow(unused_imports)]
use mightybuga_bsc::motor::MotorController;
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
