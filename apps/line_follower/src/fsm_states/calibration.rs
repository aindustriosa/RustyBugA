/// Calibration state
///
/// The calibration state is the state where the line follower is in when the user wants to calibrate
/// the line follower. Here the line follower waits for the user to press the button 1 to start the
/// calibration process.
///
/// Once the calibration is done, the line follower beeps and waits for the user to press the button 1
/// to start the line following process. The user can also press the button 2 to go back to the idle.
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
    }

    logger.log("Calibration started\r\n");
    // Calibration here
    status.board.delay.delay_ms(3000u32);

    logger.log("Calibration done\r\n");

    {
        status.board.buzzer.turn_on();
        status.board.buzzer.change_frequency(70, 1828);
        status.board.delay.delay_ms(400u32);
        status.board.buzzer.turn_off();

        status.board.delay.delay_ms(400u32);

        status.board.buzzer.turn_on();
        status.board.buzzer.change_frequency(70, 1828);
        status.board.delay.delay_ms(400u32);
        status.board.buzzer.turn_off();
    }

    logger.log("Press button 1 to start line following\r\n");
    loop {
        if status.board.btn_1.is_pressed() {
            return (FSMEvent::Button1Pressed, status);
        }
        if status.board.btn_2.is_pressed() {
            logger.log("Exit calibration\r\n");
            return (FSMEvent::Button2Pressed, status);
        }
    }
}
