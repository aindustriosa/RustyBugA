use battery_sensor_controller::BatterySensorController;
use hal_button::ButtonController;
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

use logging::Logger;

pub fn run(mut status: LineFollowerStatus) -> (FSMEvent, LineFollowerStatus) {
    let mut logger = Logger::new(&mut status.board.serial.tx);
    logger.log("Line following state\r\n");

    logger.log("Following the line!!\r\n");
    for _ in 0..100 {
        status.board.led_d1.toggle();
        status.board.led_d2.toggle();
        status.board.delay.delay_ms(50u32);

        if status.board.btn_2.is_pressed() {
            return (FSMEvent::Button2Pressed, status);
        }
        if status.board.battery_sensor.is_battery_low() {
            return (FSMEvent::BatteryIsLow, status);
        }
        if let Ok(serial_input) = status.board.serial.rx.read() {
            match serial_input {
                b'2' => {
                    return (FSMEvent::Button2Pressed, status);
                }
                _ => {}
            }
        }
    }

    logger.log("End of line following\r\n");

    (FSMEvent::NothingHappend, status)
}
