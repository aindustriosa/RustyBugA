// Example of a serial echo program using the mightybuga_bsc crate

#![no_std]
#![cfg_attr(not(doc), no_main)]
use cortex_m_rt::entry;
use panic_halt as _;
use mightybuga_bsc as board;
use nb::block;

#[entry]
fn main() -> ! {
    let mut board = board::Mightybuga_BSC::take().unwrap();
    let mut uart = board.uart;

    let s = b"\r\nPlease type characters to echo:\r\n";
    let _ = s.iter().map(|c| block!(uart.tx.write(*c))).last();

    loop {
        if let Ok(byte) = block!(uart.rx.read()) {
            let _ = block!(uart.tx.write(byte));
            //led.toggle();
        }
    }
}
