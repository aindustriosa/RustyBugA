#![no_std]
#![allow(non_camel_case_types)]

// reexport hal crates to allow users to directly refer to them
// like in https://github.com/therealprof/nucleo-f103rb/blob/master/src/lib.rs
pub use stm32f1xx_hal as hal;

use hal::serial::*;
use hal::{prelude::*};
use hal::pac::USART1;

pub use crate::hal::*;

pub struct UART{
    pub rx: Rx<USART1>,
    pub tx: Tx<USART1>,
}

pub struct Mightybuga_BSC {
    pub uart: UART,
}

impl Mightybuga_BSC {
    pub fn take() -> Result<Self, ()> {
        let dp = hal::pac::Peripherals::take().ok_or(())?;
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);
        
        let mut afio = dp.AFIO.constrain();
        let mut gpioa = dp.GPIOA.split();
        let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
        let rx = gpioa.pa10;
        let serial = Serial::new(
            dp.USART1,
            (tx, rx),
            &mut afio.mapr,
            Config::default().baudrate(115_200.bps()).wordlength_8bits().parity_none(),
            &clocks,
        );
        // get RX and TX parts
        let (tx, rx) = serial.split();
        let uart = UART { rx, tx };

        Ok(Mightybuga_BSC { uart })
    }
}