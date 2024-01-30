use crate::motor::MotorController;

// Engine gets two motors controllers, and has a way to control the car
pub struct Engine<A: MotorController, B: MotorController> {
    left: A,
    right: B,
}

impl<A: MotorController, B: MotorController> Engine<A, B> {
    pub fn new(left: A, right: B) -> Self {
        Engine { left, right }
    }

    pub fn forward(&mut self, speed: i16) {
        self.left.forward();
        self.left.set_speed(speed);
        self.right.forward();
        self.right.set_speed(speed);
    }

    pub fn backward(&mut self, speed: i16) {
        self.left.backward();
        self.left.set_speed(speed);
        self.right.backward();
        self.right.set_speed(speed);
    }

    pub fn left(&mut self, speed: i16, delta: i16) {
        self.left.forward();
        self.left.set_speed(speed - delta);
        self.right.forward();
        self.right.set_speed(speed);
    }

    pub fn right(&mut self, speed: i16, delta: i16) {
        self.left.forward();
        self.left.set_speed(speed);
        self.right.forward();
        self.right.set_speed(speed - delta);
    }

    pub fn stop(&mut self) {
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
            .with(eq(MotorState::Forward))
            .times(1)
            .returning(|_| ());
        left.expect_set_speed().times(1).returning(|_| ());

        right
            .expect_set_state()
            .with(eq(MotorState::Forward))
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
            .with(eq(MotorState::Backward))
            .times(1)
            .returning(|_| ());
        left.expect_set_speed()
            .with(eq(10))
            .times(1)
            .returning(|_| ());

        right
            .expect_set_state()
            .with(eq(MotorState::Backward))
            .times(1)
            .returning(|_| ());
        right
            .expect_set_speed()
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
        left.expect_set_speed()
            .with(eq(5))
            .times(1)
            .returning(|_| ());

        right
            .expect_set_state()
            .with(eq(MotorState::Forward))
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
            .with(eq(MotorState::Forward))
            .times(1)
            .returning(|_| ());
        left.expect_set_speed()
            .with(eq(10))
            .times(1)
            .returning(|_| ());

        right
            .expect_set_state()
            .with(eq(MotorState::Forward))
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
}
