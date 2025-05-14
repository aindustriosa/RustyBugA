/// This file contains the implementation of the battery low state of the line follower FSM.
///
/// The battery low state is the state where the line follower is in when the battery is low.
/// Here the line follower waits for the user to change the battery, it currently blinks the leds
/// D1 and D2, prints a message to the user and uses the buzzer.
///
/// If the battery is no longer low, the line follower will transition to the idle state.
///
/// The state output events are:
/// - NothingHappend: When the battery is no longer low.
use crate::board::timer::SysDelay;
use mightybuga_bsc::prelude::*;
use mightybuga_bsc::timer_based_buzzer::TimerBasedBuzzer;
use mightybuga_bsc::timer_based_buzzer::TimerBasedBuzzerInterface;

use battery_sensor_controller::BatterySensorController;

use crate::fsm::FSMEvent;
use crate::line_follower_status::LineFollowerStatus;

use logging::Logger;

pub fn run(status: & mut  LineFollowerStatus) -> FSMEvent {
    let mut logger = Logger::new(&mut status.board.serial.tx);
    logger.log("Battery low state\r\n");

    loop {
        status.board.led_d1.toggle();
        status.board.led_d2.toggle();
        logger.log("Battery is low, please change the battery\r\n");
        play_notes(&mut status.board.buzzer, &mut status.board.delay);
        status.board.delay.delay_ms(1500u32);

        if !status.board.battery_sensor.is_battery_low() {
            logger.log("Battery is no longer low\r\n");
            return FSMEvent::NothingHappened;
        }
    }
}

// Note struct defines the prescaler and counter values for a given note
struct Note {
    prescaler: u16,
    counter: u16,
}

fn play_notes(buzzer: &mut TimerBasedBuzzer, delay: &mut SysDelay) {
    const D: Note = Note {
        prescaler: 70,
        counter: 1828,
    };
    const F: Note = Note {
        prescaler: 70,
        counter: 1532,
    };

    buzzer.turn_on();
    buzzer.change_frequency(D.prescaler, D.counter);
    delay.delay_ms(2000u32);
    buzzer.change_frequency(F.prescaler, F.counter);
    delay.delay_ms(2000u32);
    buzzer.turn_off();
}
