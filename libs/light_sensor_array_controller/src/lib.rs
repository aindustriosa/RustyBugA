#![no_std]

/// The trait implemented by the light sensor arrays, to get a light map with all the values from
/// the different sensors from it.
pub trait LightSensorArrayController {
    /// Get a light map containing all the values of each sensor in the array
    fn get_light_map(&mut self) -> [u16; 8];

    /// Set led value
    fn set_led(&mut self, value: bool);
}

