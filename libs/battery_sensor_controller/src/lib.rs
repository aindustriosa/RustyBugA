#![no_std]

/// The trait implemented by the battery sensor to get the battery voltage in millivolts.
pub trait BatterySensorController {
    fn get_battery_millivolts(&mut self) -> u16;

    fn is_battery_low(&mut self) -> bool;
}
