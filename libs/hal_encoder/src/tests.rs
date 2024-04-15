use super::*;

struct MockEncoder<const BITS: u8> {
    pub steps: usize,
    pub last_steps: usize,
}

impl<const BITS: u8> MockEncoder<BITS> {
    pub fn get_max_val(&self) -> usize {
        (1 << BITS) - 1
    }

    pub fn get_min_val(&self) -> usize {
        0
    }

    pub fn simulate_move(&mut self, from: usize, to: usize) {
        self.last_steps = from;
        self.steps = to;
    }
}

impl<const BITS: u8> Default for MockEncoder<BITS> {
    fn default() -> Self {
        Self {
            steps: 0,
            last_steps: 0,
        }
    }
}

impl<const BITS: u8> EncoderController<BITS> for MockEncoder<BITS> {
    fn read_steps(&self) -> usize {
        self.steps
    }

    fn last_steps_ref(&mut self) -> &mut usize {
        &mut self.last_steps
    }

    fn reset(&mut self) {
        self.steps = 0;
        self.last_steps = 0;
    }
}

#[test]
fn delta_16bit() {
    // initial state
    let mut encoder: MockEncoder<16> = Default::default();

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
fn delta_10bit() {
    // initial state
    let mut encoder: MockEncoder<10> = Default::default();

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
fn overflow_16bit() {
    let mut encoder: MockEncoder<16> = Default::default();

    // max and min values for 16-bit encoder
    let min = encoder.get_min_val();
    assert_eq!(0, min);
    let max = encoder.get_max_val();
    assert_eq!(65535, max);

    // overflow step with 16-bit encoder
    encoder.simulate_move(max, min);
    assert_eq!(1, encoder.delta());

    // overflow with 16-bit encoder (> 1 step)
    encoder.simulate_move(max - 10, min + 10);
    assert_eq!(21, encoder.delta());
}

#[test]
fn overflow_10bit() {
    let mut encoder: MockEncoder<10> = Default::default();

    // max and min values for 10-bit encoder
    let min = encoder.get_min_val();
    assert_eq!(0, min);
    let max = encoder.get_max_val();
    assert_eq!(1023, max);

    // overflow with 10-bit encoder
    encoder.simulate_move(max, min);
    assert_eq!(1, encoder.delta());

    // overflow with 10-bit encoder (> 1 step)
    encoder.simulate_move(max - 10, min + 10);
    assert_eq!(21, encoder.delta());
}

#[test]
fn underflow_16bit() {
    let mut encoder: MockEncoder<16> = Default::default();

    // max and min values for 16-bit encoder
    let min = encoder.get_min_val();
    assert_eq!(0, min);
    let max = encoder.get_max_val();
    assert_eq!(65535, max);

    // underflow with 16-bit encoder (1 step)
    encoder.simulate_move(min, max);
    assert_eq!(-1, encoder.delta());

    // underflow with 16-bit encoder (> 1 step)
    encoder.simulate_move(min + 10, max - 10);
    assert_eq!(-21, encoder.delta());
}

#[test]
fn underflow_10bit() {
    let mut encoder: MockEncoder<10> = Default::default();

    // max and min values for 10-bit encoder
    let min = encoder.get_min_val();
    assert_eq!(0, min);
    let max = encoder.get_max_val();
    assert_eq!(1023, max);

    // underflow with 10-bit encoder (1 step)
    encoder.simulate_move(min, max);
    assert_eq!(-1, encoder.delta());

    // underflow with 10-bit encoder (> 1 step)
    encoder.simulate_move(min + 10, max - 10);
    assert_eq!(-21, encoder.delta());
}
