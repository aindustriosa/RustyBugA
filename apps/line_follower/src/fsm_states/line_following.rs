
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
        match line_position {
            Some(position) => {
                let duty = 15000;
                let delta = 2000;

                if position == 0. {
                    status.board.led_d1.set_high();
                    status.board.led_d2.set_high();
                    status.board.engine.forward(duty);
                } else if position < 0. {
                    status.board.led_d1.set_low();
                    status.board.led_d2.set_high();
                    status.board.engine.left(duty, delta);
                } else {
                    status.board.led_d1.set_high();
                    status.board.led_d2.set_low();
                    status.board.engine.right(duty, delta);
                }
                
            }
            None => {
                logger.log("No line detected\r\n");
                turn_off_robot(status);
                return FSMEvent::NothingHappened;
            }
        }

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
