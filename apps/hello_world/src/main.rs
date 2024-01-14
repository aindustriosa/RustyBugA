// Application that uses the mightybyga_bsc crate and the logging crate to print
// a message to the console.

#![no_std]
#![no_main]

use mightybuga_bsc as board;
use mightybuga_bsc::prelude::*;
use panic_halt as _;

extern crate alloc;
use crate::alloc::string::ToString;

use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use logging::Logger;

#[entry]
fn main() -> ! {
    let board = board::Mightybuga_BSC::take().unwrap();
    let mut delay = board.delay;
    let mut uart = board.uart;
    let mut led_d1 = board.leds.d1;

    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let mut logger = Logger::new(&mut uart.tx);

    let mut i: i32 = 0;
    loop {
        logger.log("Hello, world! ");
        logger.log(i.to_string().as_str());
        logger.log("\r\n");
        i += 1;
        led_d1.toggle();
        delay.delay_ms(1000_u16);
    }
}
