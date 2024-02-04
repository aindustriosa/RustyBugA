// The low lever, timer based buzzer module represents the low level interface to the buzzer.
// It provides a simple interface to control the buzzer, where you can turn it on and off.
// and also change the frequency of the sound it produces by changing the prescaler
// and the compare value of the timer.
//
// The buzzer is connected to the timer 3 channel 1. Gpio pin PB4 is connected to the buzzer.

// import the necessary from the HAL
use stm32f1xx_hal::{
    gpio::{gpiob::PB4, Alternate, PushPull},
    pac::TIM3,
};

// import the necessary from the timer based buzzer interface
use timer_based_buzzer_interface::TimerBasedBuzzerInterface;

// the struct that represents the timer based buzzer
pub struct TimerBasedBuzzer {
    // the timer peripheral
    timer: TIM3,
    // the pin connected to the buzzer
    _pin: PB4<Alternate<PushPull>>,
}

// the implementation of the timer based buzzer
impl TimerBasedBuzzer {
    pub fn new(timer: TIM3, pin: PB4<Alternate<PushPull>>) -> Self {
        // Configure the PWD peripheral at PAC level:

        // Set timer 3 mode to no divisor (72MHz), Edge-aligned, up-counting,
        // enable Auto-Reload Buffering, continous mode, disable timer.
        timer.cr1.write(|w| {
            w.arpe()
                .set_bit()
                .cms()
                .bits(0b00)
                .dir()
                .clear_bit()
                .opm()
                .clear_bit()
                .cen()
                .clear_bit()
        });

        // Timer Set Output Compare Mode to PWM Mode 1
        timer.ccmr1_output().write(|w| w.oc1m().pwm_mode1());
        // enable the channel from TIMx capture/compare enable register
        timer.ccer.write(|w| w.cc1e().set_bit());

        // Set the prescaler for note C (so the buzzer will produce an audible sound if it is turned on)
        timer.psc.write(|w| w.psc().bits(70));
        // Set the auto-reload value for note C
        let counter = 2052;
        timer.arr.write(|w| w.arr().bits(counter));
        // Set duty cycle to 50% for channel 1
        timer.ccr1().write(|w| w.ccr().bits(counter / 2));

        TimerBasedBuzzer { timer, _pin: pin }
    }
}

// the implementation of the timer based buzzer interface
impl TimerBasedBuzzerInterface for TimerBasedBuzzer {
    fn turn_on(&mut self) {
        // enable the output compare
        self.timer.ccer.write(|w| w.cc1e().set_bit());
    }

    fn turn_off(&mut self) {
        // disable the output compare
        self.timer.ccer.write(|w| w.cc1e().clear_bit());
    }

    // The frequennncy to registers formula is given by:
    // compare = f_clk / (frequency * (divisor + 1) * (prescaler + 1)) - 1
    // where f_clk is the clock frequency (72 MHz), frequency is the desired frequency,
    // divisor is the value of the timer's auto-reload register (that we always set to 0)
    // and prescaler is the value of the timer's prescaler register (for musical notes we usually set to 70).
    fn change_frequency(&mut self, prescaler: u16, compare: u16) {
        // set the prescaler
        self.timer.psc.write(|w| w.psc().bits(prescaler));
        // Set the auto-reload value for the note
        self.timer.arr.write(|w| w.arr().bits(compare));
        // Set duty cycle to 50% for channel 1
        self.timer.ccr1().write(|w| w.ccr().bits(compare / 2));
    }
}
