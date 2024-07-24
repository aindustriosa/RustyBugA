//! This is an example on how to use the `Arc` type from the `heapless` crate to allocate something
//! in the memory and then create pointers to that location in order to use it in a memory
//! safe way (except for the part in which we define the block of memory, that requires using unsafe)
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

use heapless::{
    arc_pool,
    pool::arc::ArcBlock,
};

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
    let mut gpioa = dp.GPIOA.split();

    // This macro generates all the necessary code implementations to allocate a refcell containing
    // the adc1 into a block in the memory
    arc_pool!(P: RefCell<Adc<ADC1>>);

    // Generate the memory block in which the refcell<adc1> will be allocated, we need to do
    // this in an unsafe way
    let arc_block: &'static mut ArcBlock<RefCell<Adc<ADC1>>> = unsafe {
        static mut B: ArcBlock<RefCell<Adc<ADC1>>> = ArcBlock::new();
        &mut B
    };

    // Tell the singleton to use this block in the memory for managing the refcell<adc1>
    P.manage(arc_block);

    // Allocate the adc1 inside of a refcell in the memory block we created earlier, the `alloc`
    // method doesn't have an unwrap for its Result, so we use a simple match statement to do that.
    let arc = match P.alloc(RefCell::new(Adc::adc1(dp.ADC1, clocks))) {
        Ok(adc_arc) => adc_arc,
        Err(_) => {
            panic!("Couldn't get the Arc");
        },
    };

    // Clone the pointer to the refcell<adc1>
    let arc2 = arc.clone();

    // Use any of those pointers to read using the adc
    let _x: u16 = arc2.borrow_mut().read(&mut gpioa.pa0.into_analog(&mut gpioa.crl)).unwrap();
    let _y: u16 = arc.borrow_mut().read(&mut gpioa.pa2.into_analog(&mut gpioa.crl)).unwrap();

    // Drop the pointers and the original instance of the refcell<adc1>
    drop(arc2);
    drop(arc);

    loop {

    }
}
