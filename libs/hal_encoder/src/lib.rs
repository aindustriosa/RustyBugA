#![no_std]

pub trait EncoderController {
    // This function returns the absolute step count
    fn read_steps(&self) -> u16;

    // This function returns a mutable reference to store the last step count
    fn last_steps_ref(&mut self) -> &mut u16;

    // This function resets the step count to zero.
    fn reset(&mut self);

    // This function returns the delta of the step count since the last time this function was called.
    fn delta(&mut self) -> isize {
        let steps = self.read_steps();
        let last_steps = self.last_steps_ref();
        let mut delta = steps as isize - *last_steps as isize;
        if steps >> 15 != *last_steps >> 15 {
            delta += match steps > *last_steps {
                true => -(1 << 16), // u16 underflow
                false => 1 << 16,   // u16 overflow
            }
        };
        *last_steps = steps;
        delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockEncoderController {
        pub steps: u16,
        pub last_steps: u16,
    }

    impl MockEncoderController {
        pub fn new() -> Self {
            MockEncoderController {
                steps: 0,
                last_steps: 0,
            }
        }
    }

    impl EncoderController for MockEncoderController {
        fn read_steps(&self) -> u16 {
            self.steps
        }

        fn last_steps_ref(&mut self) -> &mut u16 {
            &mut self.last_steps
        }

        fn reset(&mut self) {
            self.steps = 0;
            self.last_steps = 0;
        }
    }

    #[test]
    fn test_delta() {
        // initial state
        let mut encoder = MockEncoderController::new();

        // no movement
        assert_eq!(encoder.delta(), 0);

        // advance 1 step
        encoder.steps = 1;
        assert_eq!(encoder.delta(), 1);

        // advance 10 steps
        encoder.steps = 11;
        assert_eq!(encoder.delta(), 10);

        // back 5 steps
        encoder.steps = 6;
        assert_eq!(encoder.delta(), -5);
    }

    #[test]
    fn test_overflow() {
        let mut encoder = MockEncoderController::new();

        // overflow with 16-bit encoder
        encoder.last_steps = u16::MAX;
        encoder.steps = u16::MIN;
        assert_eq!(1, encoder.delta());

        // overflow with 16-bit encoder (> 1 step)
        encoder.last_steps = u16::MAX - 10;
        encoder.steps = u16::MIN + 10;
        assert_eq!(21, encoder.delta());
    }

    #[test]
    fn test_underflow() {
        let mut encoder = MockEncoderController::new();
        // underflow with 16-bit encoder (1 step)
        encoder.last_steps = u16::MIN;
        encoder.steps = u16::MAX;
        assert_eq!(-1, encoder.delta());

        // underflow with 16-bit encoder (> 1 step)
        encoder.last_steps = u16::MIN + 10;
        encoder.steps = u16::MAX - 10;
        assert_eq!(-21, encoder.delta());
    }
}
