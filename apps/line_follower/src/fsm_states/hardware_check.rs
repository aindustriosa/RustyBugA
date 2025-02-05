/// Hardware check state
///
/// In this state, a number of checks related to the integrity of the hardware are performed.
/// Currently, there are no checks implemented, so we wait for a second, do a pair of LED toggles
/// and beep the buzzer. Then we exti with the NothingHappend or BatteryIsLow event.
///
/// The state output events are:
/// - BatteryIsLow: When the battery is low.
/// - NothingHappend: When all checks are done.
use crate::board::timer::SysDelay;
use battery_sensor_controller::BatterySensorController;
use mightybuga_bsc::prelude::*;
use mightybuga_bsc::timer_based_buzzer::TimerBasedBuzzer;
use mightybuga_bsc::timer_based_buzzer::TimerBasedBuzzerInterface;

use crate::fsm::FSMEvent;
use crate::line_follower_status::LineFollowerStatus;

use logging::Logger;

pub fn run(status: & mut  LineFollowerStatus) -> FSMEvent {
    let mut logger = Logger::new(&mut status.board.serial.tx);
    logger.log("Hardware check state\r\n");

    {
        logger.log("Checking hardware 1\r\n");
        for _ in 0..5 {
            status.board.led_d1.toggle();
            status.board.delay.delay_ms(100u32);
        }
    }

    {
        logger.log("Checking hardware 2\r\n");
        for _ in 0..5 {
            status.board.led_d2.toggle();
            status.board.delay.delay_ms(100u32);
        }
    }

    {
        logger.log("Hardware check done\r\n");

        beep(&mut status.board.buzzer, &mut status.board.delay);
        status.board.delay.delay_ms(50u32);
        beep(&mut status.board.buzzer, &mut status.board.delay);
    }

    if status.board.battery_sensor.is_battery_low() {
        logger.log("Battery is low\r\n");
        FSMEvent::BatteryIsLow
    } else {
        FSMEvent::NothingHappend
    }
}

fn beep(buzzer: &mut TimerBasedBuzzer, delay: &mut SysDelay) {
    buzzer.turn_on();
    buzzer.change_frequency(70, 1828);
    delay.delay_ms(100u32);
    buzzer.turn_off();
}
