use crate::{
    hal::{
        gpio::{Analog, Pin},
        prelude::_embedded_hal_adc_OneShot,
    },
    ADC_POOL,
};
use battery_sensor_controller::BatterySensorController;
use heapless::pool::arc::Arc;

/// The battery sensor used to detect the battery level.
/// It uses 1 analog pin connected to the resistor divider and the ADC1 to read the voltage.
pub struct BatterySensor {
    /// The pin for the battery sensor
    pub sensor_0: Pin<'B', 0, Analog>,

    pub adc: Arc<ADC_POOL>,
}

impl BatterySensorController for BatterySensor {
    fn get_battery_millivolts(&mut self) -> u16 {
        let mut adc = self.adc.borrow_mut();

        // Read the battery voltage, take 10 samples and average them (later, so we don't lose precision):
        let mut battery_voltage_by_10: u32 = 0;
        for _ in 0..10 {
            let sample: u16 = adc.read(&mut self.sensor_0).unwrap();
            battery_voltage_by_10 += sample as u32;
        }

        // Convert the voltage to millivolts. We multiply the raw value by the battery voltage and
        // divide by the resister divider. The maximum value is 8.4V and the resister divider is
        // 47k and 20k. The raw value is 12 bits, so the maximum value is 4096 that corresponds to
        // 2.5 volts in the ADC. The formula is:
        let raw_to_millivolts_multiplier_by_1000 = 2857;
        let battery_voltage_mv =
            (battery_voltage_by_10 * raw_to_millivolts_multiplier_by_1000 / 10000) as u16;

        battery_voltage_mv
    }

    fn is_battery_low(&mut self) -> bool {
        let battery_voltage_mv = self.get_battery_millivolts();
        battery_voltage_mv < 4900
    }
}
