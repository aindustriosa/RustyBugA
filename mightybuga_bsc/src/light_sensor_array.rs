use crate::{
    hal::{
        gpio::{Analog, Output, Pin},
        prelude::_embedded_hal_adc_OneShot,
    },
    ADC_POOL,
};
use heapless::pool::arc::Arc;

/// The LineSensor used to detect the place where the line is located.
/// It uses 8 analog pins connected to the light intensity sensors, 1 pin for turning on the led in
/// the sensor array and the ADC1 to read the voltage from the sensors.
pub struct LightSensorArray {
    /// The output pin used to set the led in the sensor array high
    pub led: Pin<'B', 1, Output>,

    /// The 8 pins for the light intensity sensors
    pub sensor_0: Pin<'A', 0, Analog>, // this sensor is located on the left side of the robot
    pub sensor_1: Pin<'A', 1, Analog>,
    pub sensor_2: Pin<'A', 2, Analog>,
    pub sensor_3: Pin<'A', 3, Analog>,
    pub sensor_4: Pin<'A', 4, Analog>,
    pub sensor_5: Pin<'A', 5, Analog>,
    pub sensor_6: Pin<'A', 6, Analog>,
    pub sensor_7: Pin<'A', 7, Analog>, // this sensor is located on the right side of the robot

    pub adc: Arc<ADC_POOL>,
}

impl light_sensor_array_controller::LightSensorArrayController for LightSensorArray {
    fn get_light_map(&mut self) -> [u16; 8] {
        let mut adc = self.adc.borrow_mut();

        let light_map = [
            adc.read(&mut self.sensor_0).unwrap(),
            adc.read(&mut self.sensor_1).unwrap(),
            adc.read(&mut self.sensor_2).unwrap(),
            adc.read(&mut self.sensor_3).unwrap(),
            adc.read(&mut self.sensor_4).unwrap(),
            adc.read(&mut self.sensor_5).unwrap(),
            adc.read(&mut self.sensor_6).unwrap(),
            adc.read(&mut self.sensor_7).unwrap(),
        ];

        light_map
    }

    fn set_led(&mut self, value: bool) -> () {
        match value {
            true => self.led.set_high(),
            false => self.led.set_low(),
        }
    }
}
