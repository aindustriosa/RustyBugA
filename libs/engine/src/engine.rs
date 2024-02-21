use crate::motor::MotorController;

// Engine gets two motors controllers, and has a way to control the car
pub struct Engine<A: MotorController, B: MotorController> {
    left: A,
    right: B,
}

pub trait EngineController {
    fn forward(&mut self, duty: u16);
    fn backward(&mut self, duty: u16);
    fn left(&mut self, duty: u16, delta: u16);
    fn right(&mut self, duty: u16, delta: u16);
    fn stop(&mut self);
}

impl<A: MotorController, B: MotorController> Engine<A, B> {
    pub fn new(left: A, right: B) -> Self {
        Engine { left, right }
    }
}

impl<A: MotorController, B: MotorController> EngineController for Engine<A, B> {
    // duty goes from 0 to 65535
    fn forward(&mut self, duty: u16) {
        self.left.forward();
        self.left.set_duty(duty);
        self.right.forward();
        self.right.set_duty(duty);
    }

    // duty goes from 0 to 65535
    fn backward(&mut self, duty: u16) {
        self.left.backward();
        self.left.set_duty(duty);
        self.right.backward();
        self.right.set_duty(duty);
    }

    // duty goes from 0 to 65535
    // delta goes from 0 to 65535
    // if duty is 10 and delta is 5, the left motor will have a duty of 5 and the right motor will have a duty of 10
    fn left(&mut self, duty: u16, delta: u16) {
        self.left.forward();
        self.left.set_duty(duty - delta);
        self.right.forward();
        self.right.set_duty(duty);
    }

    // duty goes from 0 to 65535
    // delta goes from 0 to 65535
    // if duty is 10 and delta is 5, the left motor will have a duty of 10 and the right motor will have a duty of 5
    fn right(&mut self, duty: u16, delta: u16) {
        self.left.forward();
        self.left.set_duty(duty);
        self.right.forward();
        self.right.set_duty(duty - delta);
    }

    fn stop(&mut self) {
        self.left.stop();
        self.right.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::motor::MotorState;

    use mockall::predicate::*;
    use mockall::*;

    mock! {
      FakeMotor {}

      impl MotorController for FakeMotor {
        fn set_state(&mut self, state: MotorState);
        fn set_duty(&mut self, duty: u16);
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
            .with(eq(MotorState::Forward))
            .times(1)
            .returning(|_| ());
        left.expect_set_duty().times(1).returning(|_| ());

        right
            .expect_set_state()
            .with(eq(MotorState::Forward))
            .times(1)
            .returning(|_| ());
        right.expect_set_duty().times(1).returning(|_| ());

        // when
        let mut engine = Engine::new(left, right);
        engine.forward(10);
    }

    #[test]
    fn test_engine_backward() {
        // given
        let (mut left, mut right) = get_motors();
        left.expect_set_state()
            .with(eq(MotorState::Backward))
            .times(1)
            .returning(|_| ());
        left.expect_set_duty()
            .with(eq(10))
            .times(1)
            .returning(|_| ());

        right
            .expect_set_state()
            .with(eq(MotorState::Backward))
            .times(1)
            .returning(|_| ());
        right
            .expect_set_duty()
            .with(eq(10))
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
            .with(eq(MotorState::Forward))
            .times(1)
            .returning(|_| ());
        left.expect_set_duty()
            .with(eq(5))
            .times(1)
            .returning(|_| ());

        right
            .expect_set_state()
            .with(eq(MotorState::Forward))
            .times(1)
            .returning(|_| ());
        right
            .expect_set_duty()
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
            .with(eq(MotorState::Forward))
            .times(1)
            .returning(|_| ());
        left.expect_set_duty()
            .with(eq(10))
            .times(1)
            .returning(|_| ());

        right
            .expect_set_state()
            .with(eq(MotorState::Forward))
            .times(1)
            .returning(|_| ());
        right
            .expect_set_duty()
            .with(eq(5))
            .times(1)
            .returning(|_| ());

        // when
        let mut engine = Engine::new(left, right);
        engine.right(10, 5);
    }
}
