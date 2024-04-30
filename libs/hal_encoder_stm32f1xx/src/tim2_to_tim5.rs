pub use hal_encoder::EncoderController;
use stm32f1xx_hal::pac::tim2;

pub struct IncrementalEncoder {
    tim: *const tim2::RegisterBlock,
    last_steps: usize,
}

pub enum TimerChannels {
    Ch1Ch2,
    Ch3Ch4,
}

pub enum EncoderPolarity {
    PolarityAB,
    PolarityBA,
}

impl IncrementalEncoder {
    const SLAVE_MODE_SELECTION: u8 = tim2::smcr::SMS_A::EncoderMode3 as u8;

    pub fn new(tim: *const tim2::RegisterBlock, channels: TimerChannels, polarity: EncoderPolarity) -> Self {
        unsafe {
            // up/down on TI1FP1+TI2FP2 edges depending on complementary input
            (*tim).smcr.modify(|_, w| w.sms().bits(IncrementalEncoder::SLAVE_MODE_SELECTION));

            // quadrature encoder mode, input capture channels
            match channels {
                TimerChannels::Ch1Ch2 => {
                    (*tim).ccmr1_input().modify(|_, w| w.cc1s().ti1().cc2s().ti2());
                }
                TimerChannels::Ch3Ch4 => {
                    (*tim).ccmr2_input().modify(|_, w| w.cc3s().ti3().cc4s().ti4());
                }
            }

            // polarity of the input channels
            match polarity {
                EncoderPolarity::PolarityAB => {
                    (*tim).ccer.modify(|_, w| w.cc1p().clear_bit().cc2p().clear_bit());
                }
                EncoderPolarity::PolarityBA => {
                    (*tim).ccer.modify(|_, w| w.cc1p().set_bit().cc2p().clear_bit());
                }
            }

            // initial value to the middle
            (*tim).cnt.modify(|_, w| w.bits(0));

            // auto-reload value to the maximum
            (*tim).arr.modify(|_, w| w.bits(u16::MAX as u32));
        }

        Self { tim, last_steps: 0 }
    }

    // return the current number of steps
    pub fn get_steps(&self) -> u16 {
        unsafe {
            // read the counter register
            (*self.tim).cnt.read().cnt().bits() as u16
        }
    }

    // set the current number of steps
    pub fn set_steps(&mut self, steps: u16) {
        unsafe {
            // set the counter register
            (*self.tim).cnt.write(|w| w.bits(steps as u32));
        }
    }

    // enable the encoder
    pub fn enable(&mut self) {
        unsafe {
            // set the counter enable bit
            (*self.tim).cr1.modify(|_, w| w.cen().set_bit());
        }
    }

    // disable the encoder
    pub fn disable(&mut self) {
        unsafe {
            // clear the counter enable bit
            (*self.tim).cr1.modify(|_, w| w.cen().clear_bit());
        }
    }
}

impl EncoderController<16> for IncrementalEncoder {
    fn steps(&self) -> usize {
        self.get_steps() as usize
    }

    fn reset(&mut self) {
        self.set_steps(0)
    }

    fn last_steps_ref(&mut self) -> &mut usize {
        &mut self.last_steps
    }
}
