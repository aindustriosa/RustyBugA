#![no_std]
#![cfg_attr(not(doc), no_main)]
use core::cell::RefCell;

use stm32f1xx_hal::{
    pac::ADC1,
    adc::Adc,
};

use panic_halt as _;

use mightybuga_bsc as board;

use cortex_m_rt::entry;
use board::hal::{pac, prelude::*};

use heapless::arc_pool;

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let _cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let _gpioc = dp.GPIOC.split();

    arc_pool!(P: RefCell<Adc<ADC1>>);

    let arc = match P.alloc(RefCell::new(Adc::adc1(dp.ADC1, clocks))) {
        Ok(adc_arc) => adc_arc,
        Err(_) => panic!("Couldn't get the adc arc"),
    };
    let arc2 = arc.clone();

    let mut gpioa = dp.GPIOA.split();
    let _x: u16 = arc2.borrow_mut().read(&mut gpioa.pa0.into_analog(&mut gpioa.crl)).unwrap();

    drop(arc2);
    drop(arc);

    loop {

    }
}
