
/// Line following state
///
/// This state is responsible for following the line.
///
/// Currently it does nothing, just logs the state, waits for a second and returns the proper event.
///
/// The state output events are:
/// - Button2Pressed: When the user presses the button 2 (the user wants to end the state).
/// - NothingHappend: When nothing happened, the line follower reaches a estate of end of following the line.
/// - BatteryIsLow: When the battery is low.
use mightybuga_bsc::prelude::*;

use crate::fsm::FSMEvent;
use crate::line_follower_status::LineFollowerStatus;

use light_sensor_array_controller::LightSensorArrayController;
use battery_sensor_controller::BatterySensorController;
use engine::engine::EngineController;
use hal_button::ButtonController;
use logging::Logger;

// Direction enum to represent the direction of the engine
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
    Stop,
}

// struct with telemetry info to be printed
struct TelemetryData{
    pub line_position: Option<f32>,
    pub line_sensor: [u16; 8],
    pub engine_duty: u16,
    pub engine_direction: Direction,
    pub engine_delta: u16,
    pub battery_millivolts: u16,
}

pub fn run(status: &mut LineFollowerStatus) -> FSMEvent {
    let mut logger = Logger::new(&mut status.board.serial.tx);
    logger.log("Line following state\r\n");

    // First, the line follower will wait for 5 seconds before starting to move. It will allow button 2 to be pressed to stop the line follower.
    // and monitor the battery level. Button 1 exits the loop.
    logger.log("Waiting for 5 seconds before starting to move\r\n");
    for _ in 0..90 {
        status.board.led_d1.toggle();
        status.board.led_d2.toggle();
        status.board.delay.delay_ms(50u32);

        if status.board.btn_1.is_pressed() {
            break;
        }

        if status.board.btn_2.is_pressed() {
            return FSMEvent::Button2Pressed;
        }
        if status.board.battery_sensor.is_battery_low() {
            return FSMEvent::BatteryIsLow;
        }
        if let Ok(serial_input) = status.board.serial.rx.read() {
            match serial_input {
                b'1' => {
                    break;
                }
                b'2' => {
                    return FSMEvent::Button2Pressed;
                }
                _ => {}
            }
        }
    }
    logger.log("turn on line sensor led\r\n");
    status.board.light_sensor_array.set_led(true);

    // Now, the line follower will follow the line with a simple algorithm:
    // - If the line is in the middle of the sensors, it will move forward.
    // - If the line is on the left sensors, it will turn left.
    // - If the line is on the right sensors, it will turn right.
    // - If there is no line, it will stop.
    // - If the button 2 is pressed, it will stop.
    // - If the battery is low, it will stop.
    loop {
        let line_sensor = status.board.light_sensor_array.get_light_map();
        let line_position = get_line_position(line_sensor);
        let mut telemetry_data = TelemetryData {
            line_position,
            line_sensor,
            engine_duty: 0,
            engine_direction: Direction::Stop,
            engine_delta: 0,
            battery_millivolts: status.board.battery_sensor.get_battery_millivolts(),
        };
        match line_position {
            Some(position) => {
                let duty = 15000;
                telemetry_data.engine_duty = duty;
                let delta = 2000;
                telemetry_data.engine_delta = delta;

                if position == 0. {
                    status.board.led_d1.set_high();
                    status.board.led_d2.set_high();
                    status.board.engine.forward(duty);
                    telemetry_data.engine_direction = Direction::Forward;
                } else if position < 0. {
                    status.board.led_d1.set_low();
                    status.board.led_d2.set_high();
                    status.board.engine.left(duty, delta);
                    telemetry_data.engine_direction = Direction::Left;
                } else {
                    status.board.led_d1.set_high();
                    status.board.led_d2.set_low();
                    status.board.engine.right(duty, delta);
                    telemetry_data.engine_direction = Direction::Right;
                }
                
            }
            None => {
                logger.log("No line\r\n");
                turn_off_robot(status);
                telemetry_data.engine_direction = Direction::Stop;
                return FSMEvent::NothingHappened;
            }
        }

        telemetry_data.battery_millivolts = status.board.battery_sensor.get_battery_millivolts();
        log_telemetry(&mut logger, &telemetry_data);
        status.board.delay.delay_ms(50u32);

        if status.board.btn_2.is_pressed() {
            turn_off_robot(status);
            return FSMEvent::Button2Pressed;
        }
        if status.board.battery_sensor.is_battery_low() {
            turn_off_robot(status);
            return FSMEvent::BatteryIsLow;
        }
        if let Ok(serial_input) = status.board.serial.rx.read() {
            match serial_input {
                b'2' => {
                    turn_off_robot(status);
                    return FSMEvent::Button2Pressed;
                }
                _ => {}
            }
        }
    }
}

fn turn_off_robot(status: &mut LineFollowerStatus) {
    status.board.engine.stop();
    status.board.led_d1.set_low();
    status.board.led_d2.set_low();
    status.board.light_sensor_array.set_led(false);
}

// get_line_position function returns a value between -1 and 1, where -1 means the line is on the left, 0 means the line is in the middle, and 1 means the line is on the right.
// The line sensor is an array of 8 values, where each value represents the light intensity of a sensor. The higher the value, the less light is detected (the line is black).
// The function calculates the position of the line as the position of the sensor with the lowest value.
// If no line is detected, the function returns None.
fn get_line_position(line_sensor: [u16; 8]) -> Option<f32> {
    let mut max_value = line_sensor[0];
    let mut max_index = 0;
    for i in 1..8 {
        if line_sensor[i] > max_value {
            max_value = line_sensor[i];
            max_index = i;
        }
    }

    let max_threshold = 1000;
    if max_value < max_threshold {
        return None;
    }

    match max_index {
        0 => Some(1.),   // line is on the right
        1 => Some(0.75),
        2 => Some(0.5),
        3 => Some(0.0),
        4 => Some(0.0),
        5 => Some(-0.5),
        6 => Some(-0.75),
        7 => Some(-1.),  // line is on the left
        _ => None,
    }
}

// log_telemetry function logs the telemetry data to the logger
// Note that we do not have format, we only have log for &str and
// log_u16 for u16 values.
// This function will log the telemetry data in a simple format.
// The format is:
// <line_position*100>, <line_sensor_0>, <line_sensor_1>, ..., <line_sensor_7>, <engine_duty>, <engine_direction>, <engine_delta>, <battery_millivolts>
fn log_telemetry(logger: &mut Logger, telemetry_data: &TelemetryData) {
    logger.log("Telemetry: ");
    
    if let Some(line_position) = telemetry_data.line_position {
        let line_position_sign = if line_position < 0.0 { "-" } else { "" };

        let line_position_uint = (line_position.abs() * 100.0) as u16;
        logger.log(line_position_sign);
        logger.log_u16(&line_position_uint);
    } else {
        logger.log("None");
    }
    
    logger.log(", [");
    for (index, value) in telemetry_data.line_sensor.iter().enumerate() {
        logger.log_u16(value);
        if index != telemetry_data.line_sensor.len() - 1 {
            logger.log(", ");
        }
    }
    
    logger.log("], ");
    logger.log_u16(&telemetry_data.engine_duty);
    
    match telemetry_data.engine_direction {
        Direction::Forward => logger.log("Forward"),
        Direction::Backward => logger.log("Backward"),
        Direction::Left => logger.log("Left"),
        Direction::Right => logger.log("Right"),
        Direction::Stop => logger.log("Stop"),
    }
    
    logger.log(", ");
    logger.log_u16(&telemetry_data.engine_delta);
    logger.log(", ");
    logger.log_u16(&telemetry_data.battery_millivolts);
    
    logger.log("\r\n");
}