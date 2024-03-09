use crate::hal::{
    gpio::{
        Pin,
        Analog,
        Output,
    },
    pac::ADC1,
    adc::Adc,
    prelude::_embedded_hal_adc_OneShot,
};

/// The LineSensor used to detect the place where the line is located.
/// It uses 8 analog pins connected to the light intensity sensors, 1 pin for turning on the led in
/// the sensor array and the ADC1 to read the voltage from the sensors.
pub struct LineSensor {
    /// The output pin used to set the led in the sensor array high
    pub led: Pin<'B', 1, Output>,

    /// The 8 pins for the light intensity sensors
    pub sensor_0: Pin<'A', 0, Analog>,
    pub sensor_1: Pin<'A', 1, Analog>,
    pub sensor_2: Pin<'A', 2, Analog>,
    pub sensor_3: Pin<'A', 3, Analog>,
    pub sensor_4: Pin<'A', 4, Analog>,
    pub sensor_5: Pin<'A', 5, Analog>,
    pub sensor_6: Pin<'A', 6, Analog>,
    pub sensor_7: Pin<'A', 7, Analog>,

    pub adc: Adc<ADC1>,
}

impl LineSensor {
    pub fn get_line_map(&mut self) -> [u16; 8] {
        self.led.set_high();

        let light_map = [
            self.adc.read(&mut self.sensor_0).unwrap(),
            self.adc.read(&mut self.sensor_1).unwrap(),
            self.adc.read(&mut self.sensor_2).unwrap(),
            self.adc.read(&mut self.sensor_3).unwrap(),
            self.adc.read(&mut self.sensor_4).unwrap(),
            self.adc.read(&mut self.sensor_5).unwrap(),
            self.adc.read(&mut self.sensor_6).unwrap(),
            self.adc.read(&mut self.sensor_7).unwrap(),
        ];

        self.led.set_low();

        light_map
    }
}

