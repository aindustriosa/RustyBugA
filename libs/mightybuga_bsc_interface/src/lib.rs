#![no_std]

pub use core::convert::Infallible;
pub use embedded_hal::blocking::delay::DelayMs;
pub use embedded_hal::blocking::serial::Write;
pub use embedded_hal::serial::Read;
pub use embedded_hal::digital::v2::{OutputPin, InputPin};

pub trait MightybugaBsc {
    // LED types
    type LedD1: OutputPin;
    type LedD2: OutputPin;
    
    // Serial type
    type Serial: Serial;

    // Delay type
    type Delay: DelayMs<u32>;
    
    // Button types
    type Btn1: InputPin;
    type Btn2: InputPin;
    type Btn3: InputPin;
    
    // Buzzer type
    type Buzzer: BuzzerTrait;
    
    // Engine type
    type Engine: EngineTrait;
    
    // Sensor types
    type LightSensorArray: LightSensorTrait;
    type BatterySensor: BatterySensorTrait;
    
    // Encoder types
    type EncoderL: EncoderTrait;
    type EncoderR: EncoderTrait;

    // Getter methods
    fn led_d1(&mut self) -> &mut Self::LedD1;
    fn led_d2(&mut self) -> &mut Self::LedD2;
    fn serial(&mut self) -> &mut Self::Serial;
    fn delay(&mut self) -> &mut Self::Delay;
    fn btn_1(&self) -> &Self::Btn1;
    fn btn_2(&self) -> &Self::Btn2;
    fn btn_3(&self) -> &Self::Btn3;
    fn buzzer(&mut self) -> &mut Self::Buzzer;
    fn engine(&mut self) -> &mut Self::Engine;
    fn light_sensor_array(&mut self) -> &mut Self::LightSensorArray;
    fn battery_sensor(&mut self) -> &mut Self::BatterySensor;
    fn encoder_l(&mut self) -> &mut Self::EncoderL;
    fn encoder_r(&mut self) -> &mut Self::EncoderR;
}

pub trait Serial{
    type Read: Read<u8>;
    type Write: Write<u8, Error = Infallible>;
    
    fn rx(&mut self) -> &mut Self::Read;
    fn tx(&mut self) -> &mut Self::Write;
}

// Define custom traits for non-standard peripherals
pub trait BuzzerTrait {
    type Error;
    fn beep(&mut self, frequency: u32, duration_ms: u32) -> Result<(), Self::Error>;
}

pub trait EngineTrait {
    type Error;
    fn move_robot(&mut self, left_speed: i16, right_speed: i16) -> Result<(), Self::Error>;
    fn stop(&mut self) -> Result<(), Self::Error>;
}

pub trait LightSensorTrait {
    type Error;
    fn read_sensors(&mut self) -> Result<[u16; 8], Self::Error>;
}

pub trait BatterySensorTrait {
    type Error;
    fn read_voltage(&mut self) -> Result<u16, Self::Error>;
}

pub trait EncoderTrait {
    fn count(&self) -> i32;
    fn reset(&mut self);
}
