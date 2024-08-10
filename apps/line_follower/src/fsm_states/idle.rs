/// Iddle state
///
/// This state is the initial state of the line follower, and it is the state where the line follower
/// waits for the user to initiate the different states.
///
/// The state prints a menu to the user and waits for the following actions:
///  - button 1 pressed: go to hardware check state
///  - button 2 pressed: go to calibration state
///  - battery is low: go to battery low state
///
/// During this state the led D1 is on and the led D2 is off.
///
/// The state output events are:
/// - Button1Pressed: When the user presses the button 1.
/// - Button2Pressed: When the user presses the button 2.
/// - BatteryIsLow: When the battery is low.
use battery_sensor_controller::BatterySensorController;
use hal_button::ButtonController;

use crate::fsm::FSMEvent;
use crate::line_follower_status::LineFollowerStatus;

use logging::Logger;

pub fn run(status: &mut LineFollowerStatus) -> FSMEvent {
    let mut logger = Logger::new(&mut status.board.serial.tx);
    logger.log("Idle state\r\n");
    status.board.led_d1.set_high();
    status.board.led_d2.set_low();

    print_menu(&mut logger);

    loop {
        if status.board.btn_1.is_pressed() {
            return FSMEvent::Button1Pressed;
        }

        if status.board.btn_2.is_pressed() {
            return FSMEvent::Button2Pressed;
        }

        if status.board.battery_sensor.is_battery_low() {
            return FSMEvent::BatteryIsLow;
        }

        if let Ok(byte) = status.board.serial.rx.read() {
            match byte {
                b'1' => {
                    return FSMEvent::Button1Pressed;
                }
                b'2' => {
                    return FSMEvent::Button2Pressed;
                }
                b'l' => {
                    return FSMEvent::BatteryIsLow;
                }
                _ => {
                    logger.log("Invalid input\r\n");
                }
            }
        }
    }
}

fn print_menu(logger: &mut Logger) {
    logger.log("Menu:\r\n");
    logger.log(" press button 1 to go to hardware check state\r\n");
    logger.log(" press button 2 to go to calibration state\r\n");
    logger.log(" you can also use the following keys:\r\n");
    logger.log(" press '1' to go to hardware check state\r\n");
    logger.log(" press '2' to go to calibration state\r\n");
    logger.log(" press 'l' to go to battery low state\r\n");
}
