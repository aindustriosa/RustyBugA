/// Calibration state
///
/// The calibration state is the state where the line follower is in when the user wants to calibrate
/// the line follower. Here the line follower waits for the user to press the button 1 to start the
/// calibration process.
///
/// Once the calibration is done, the line follower beeps and waits for the user inputs.
///
/// The user can also use the serial rx to send the button 1 or button 2 command to the line follower.
///
/// The state output events are:
/// - Button1Pressed: When the user presses the button 1.
/// - Button2Pressed: When the user presses the button 2.
/// - BatteryIsLow: When the battery is low.
///
use crate::board::timer::SysDelay;
use hal_button::ButtonController;
use mightybuga_bsc::prelude::*;
use mightybuga_bsc::timer_based_buzzer::TimerBasedBuzzer;
use mightybuga_bsc::timer_based_buzzer::TimerBasedBuzzerInterface;

use crate::fsm::FSMEvent;
use crate::line_follower_status::LineFollowerStatus;

use logging::Logger;

pub fn run(mut status: LineFollowerStatus) -> (FSMEvent, LineFollowerStatus) {
    let mut logger = Logger::new(&mut status.board.serial.tx);
    logger.log("Calibration state\r\n");

    logger.log("Press button 1 to start calibration\r\n");

    // Wait for buttons to be released before waiting for buttons to be pressed
    status.board.delay.delay_ms(2000u32);

    loop {
        if status.board.btn_1.is_pressed() {
            break;
        }
        if status.board.btn_2.is_pressed() {
            logger.log("Exit calibration\r\n");
            return (FSMEvent::Button2Pressed, status);
        }
        if let Ok(serial_input) = status.board.serial.rx.read() {
            match serial_input {
                b'1' => break,
                b'2' => {
                    logger.log("Exit calibration\r\n");
                    return (FSMEvent::Button2Pressed, status);
                }
                _ => {}
            }
        }
    }

    logger.log("Calibration started\r\n");
    // Calibration here
    status.board.delay.delay_ms(3000u32);

    logger.log("Calibration done\r\n");

    beep(&mut status.board.buzzer, &mut status.board.delay);
    status.board.delay.delay_ms(50u32);
    beep(&mut status.board.buzzer, &mut status.board.delay);

    logger.log("Press button 1 to start line following\r\n");
    logger.log("Press button 2 to go back to idle\r\n");
    loop {
        if status.board.btn_1.is_pressed() {
            return (FSMEvent::Button1Pressed, status);
        }
        if status.board.btn_2.is_pressed() {
            logger.log("Exit calibration\r\n");
            return (FSMEvent::Button2Pressed, status);
        }
        if let Ok(serial_input) = status.board.serial.rx.read() {
            match serial_input {
                b'1' => return (FSMEvent::Button1Pressed, status),
                b'2' => return (FSMEvent::Button2Pressed, status),
                _ => {}
            }
        }
    }
}

fn beep(buzzer: &mut TimerBasedBuzzer, delay: &mut SysDelay) {
    buzzer.turn_on();
    buzzer.change_frequency(70, 1828);
    delay.delay_ms(100u32);
    buzzer.turn_off();
}
