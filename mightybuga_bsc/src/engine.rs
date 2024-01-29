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
        self.left.start();
        self.left.set_speed(speed);
        self.right.start();
        self.right.set_speed(speed);
    }

    pub fn backward(&mut self, speed: i16) {
        self.left.start();
        self.left.set_speed(0 - speed);
        self.right.start();
        self.right.set_speed(0 - speed);
    }

    pub fn left(&mut self, speed: i16, delta: i16) {
        self.left.start();
        self.left.set_speed(speed - delta);
        self.right.start();
        self.right.set_speed(speed);
    }

    pub fn right(&mut self, speed: i16, delta: i16) {
        self.left.start();
        self.left.set_speed(speed);
        self.right.start();
        self.right.set_speed(speed - delta);
    }

    pub fn stop(&mut self) {
        self.left.stop();
        self.right.stop();
    }
}
