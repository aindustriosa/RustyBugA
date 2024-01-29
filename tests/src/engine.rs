#[allow(unused_imports)]
use mightybuga_bsc::engine::Engine;
use mightybuga_bsc::motor::MotorController;
use mightybuga_bsc::motor::MotorState;
use mockall::predicate::*;
use mockall::*;

mock! {
  FakeMotor {}

  impl MotorController for FakeMotor {
    fn set_state(&mut self, state: MotorState);
    fn set_speed(&mut self, speed: i16);
  }

}

#[allow(dead_code)]
fn get_motors() -> (MockFakeMotor, MockFakeMotor) {
    (MockFakeMotor::new(), MockFakeMotor::new())
}

#[test]
fn test_engine_forward() {
    // given
    let (mut left, mut right) = get_motors();
    left.expect_set_state()
        .with(eq(MotorState::Start))
        .times(1)
        .returning(|_| ());
    left.expect_set_speed().times(1).returning(|_| ());

    right
        .expect_set_state()
        .with(eq(MotorState::Start))
        .times(1)
        .returning(|_| ());
    right.expect_set_speed().times(1).returning(|_| ());

    // when
    let mut engine = Engine::new(left, right);
    engine.forward(10);
}

#[test]
fn test_engine_backward() {
    // given
    let (mut left, mut right) = get_motors();
    left.expect_set_state()
        .with(eq(MotorState::Start))
        .times(1)
        .returning(|_| ());
    left.expect_set_speed()
        .with(eq(-10))
        .times(1)
        .returning(|_| ());

    right
        .expect_set_state()
        .with(eq(MotorState::Start))
        .times(1)
        .returning(|_| ());
    right
        .expect_set_speed()
        .with(eq(-10))
        .times(1)
        .returning(|_| ());

    // when
    let mut engine = Engine::new(left, right);
    engine.backward(10);
}

#[test]
fn test_engine_left() {
    // given
    let (mut left, mut right) = get_motors();
    left.expect_set_state()
        .with(eq(MotorState::Start))
        .times(1)
        .returning(|_| ());
    left.expect_set_speed()
        .with(eq(5))
        .times(1)
        .returning(|_| ());

    right
        .expect_set_state()
        .with(eq(MotorState::Start))
        .times(1)
        .returning(|_| ());
    right
        .expect_set_speed()
        .with(eq(10))
        .times(1)
        .returning(|_| ());

    // when
    let mut engine = Engine::new(left, right);
    engine.left(10, 5);
}

#[test]
fn test_engine_rigth() {
    // given
    let (mut left, mut right) = get_motors();
    left.expect_set_state()
        .with(eq(MotorState::Start))
        .times(1)
        .returning(|_| ());
    left.expect_set_speed()
        .with(eq(10))
        .times(1)
        .returning(|_| ());

    right
        .expect_set_state()
        .with(eq(MotorState::Start))
        .times(1)
        .returning(|_| ());
    right
        .expect_set_speed()
        .with(eq(5))
        .times(1)
        .returning(|_| ());

    // when
    let mut engine = Engine::new(left, right);
    engine.right(10, 5);
}
