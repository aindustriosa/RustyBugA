// Example of use of the usart peripheral with the stm32f1xx_hal crate
//

#![no_std]
#![cfg_attr(not(doc), no_main)]

use mightybuga_bsc as board;

use cortex_m_rt::entry;
use nb::block;
use stm32f1xx_hal::pac::USART1;
use stm32f1xx_hal::{pac, prelude::*, serial::*};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let mut afio = dp.AFIO.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // create a delay abstraction based on SysTick
    let mut delay = cp.SYST.delay(&clocks);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split();

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // turn on the led
    led.set_high();

    // Acquire the GPIOA peripheral
    let mut gpioa = dp.GPIOA.split();

    // Configure gpio A pins 9 and 10 as a push-pull output. The `crh` register is passed to the
    // function in order to configure the port. For pins 0-7, crl should be passed instead.
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    // Configure the serial peripheral
    let serial = Serial::new(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        Config::default()
            .baudrate(115_200.bps())
            .wordlength_8bits()
            .parity_none(),
        &clocks,
    );

    // Split the serial struct into a receiving and a transmitting part
    let (mut tx, mut rx) = serial.split();

    /* Print a nice hello message */
    let s = b"\r\nPlease type characters to echo:\r\n";

    let _ = s.iter().map(|c| block!(tx.write(*c))).last();

    loop {
        // Wait for a byte to be received and print it back if no error toggling the led
        if let Ok(byte) = block!(rx.read()) {
            let _ = block!(tx.write(byte));
            led.toggle();
        }
    }
}
