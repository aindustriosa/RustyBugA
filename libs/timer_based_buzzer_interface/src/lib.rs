// Time based buzzer interface
#![no_std]

// this trait represents the interface of the timer based buzzer
pub trait TimerBasedBuzzerInterface {
    // this function turns the buzzer on
    fn turn_on(&mut self);
    // this function turns the buzzer off
    fn turn_off(&mut self);
    // this function changes the frequency of the buzzer. The prescaler and compare
    // values are used to change the frequency of the buzzer. These values depend on the
    // timer configuration, more in concrete the speed of the clock. This speed could be
    // given in ahother function so the registers can be calculated.
    fn change_frequency(&mut self, prescaler: u16, compare: u16);
}
