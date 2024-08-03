use mightybuga_bsc::prelude::*;

use crate::fsm::FSMEvent;
use crate::line_follower_status::LineFollowerStatus;

use logging::Logger;

pub fn run(mut status: LineFollowerStatus) -> (FSMEvent, LineFollowerStatus) {
    let mut logger = Logger::new(&mut status.board.serial.tx);
    logger.log("Line following state\r\n");
    status.board.delay.delay_ms(1000u32);
    (FSMEvent::NothingHappend, status)
}
