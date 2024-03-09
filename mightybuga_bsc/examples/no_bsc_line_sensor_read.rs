#![no_std]
#![cfg_attr(not(doc), no_main)]
use board::pac::USART1;
use panic_halt as _;

use mightybuga_bsc as board;
use mightybuga_bsc::prelude::entry;

use board::adc::Adc;
use board::hal::serial::*;
use board::hal::{pac, prelude::*};
use nb::block;

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let mut afio = dp.AFIO.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut delay = cp.SYST.delay(&clocks);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split();

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // Acquire the GPIOA peripheral
    let mut gpioa = dp.GPIOA.split();

    // Get the pins to read with the ADC1, these are from PA0 to PA7. We cannot use an array to make it
    // easier to iterate over them because the pins have different types.
    let mut pa0 = gpioa.pa0.into_analog(&mut gpioa.crl);
    let mut pa1 = gpioa.pa1.into_analog(&mut gpioa.crl);
    let mut pa2 = gpioa.pa2.into_analog(&mut gpioa.crl);
    let mut pa3 = gpioa.pa3.into_analog(&mut gpioa.crl);
    let mut pa4 = gpioa.pa4.into_analog(&mut gpioa.crl);
    let mut pa5 = gpioa.pa5.into_analog(&mut gpioa.crl);
    let mut pa6 = gpioa.pa6.into_analog(&mut gpioa.crl);
    let mut pa7 = gpioa.pa7.into_analog(&mut gpioa.crl);

    // We also use PB1 to turn on the light of the line sensor
    let mut gpiob = dp.GPIOB.split();
    let mut line_sensor_led = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);

    // Get the ADC, using the ADC1 peripheral
    let mut adc = Adc::adc1(dp.ADC1, clocks);

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

    let s = b"\r\nLine sensor readings:\r\n";
    let _ = s.iter().map(|c| block!(tx.write(*c))).last();

    loop {
        line_sensor_led.set_high();
        delay.delay_ms(100_u16);

        let s = b"\r\nReadings with light on: ";
        let _ = s.iter().map(|c| block!(tx.write(*c))).last();

        // read all the pins and put the readings in an array
        let mut readings = [0_u16; 8];
        readings[0] = adc.read(&mut pa0).unwrap();
        readings[1] = adc.read(&mut pa1).unwrap();
        readings[2] = adc.read(&mut pa2).unwrap();
        readings[3] = adc.read(&mut pa3).unwrap();
        readings[4] = adc.read(&mut pa4).unwrap();
        readings[5] = adc.read(&mut pa5).unwrap();
        readings[6] = adc.read(&mut pa6).unwrap();
        readings[7] = adc.read(&mut pa7).unwrap();

        // print the readings
        print_u16_array_with_max_and_min(&mut tx, readings);

        delay.delay_ms(100_u16);

        let s = b"  Readings with light off: ";
        let _ = s.iter().map(|c| block!(tx.write(*c))).last();

        line_sensor_led.set_low();
        delay.delay_ms(100_u16);

        // read all the pins and put the readings in an array
        readings[0] = adc.read(&mut pa0).unwrap();
        readings[1] = adc.read(&mut pa1).unwrap();
        readings[2] = adc.read(&mut pa2).unwrap();
        readings[3] = adc.read(&mut pa3).unwrap();
        readings[4] = adc.read(&mut pa4).unwrap();
        readings[5] = adc.read(&mut pa5).unwrap();
        readings[6] = adc.read(&mut pa6).unwrap();
        readings[7] = adc.read(&mut pa7).unwrap();

        // print the readings
        print_u16_array_with_max_and_min(&mut tx, readings);

        //delay.delay_ms(500_u16);
        led.toggle();
    }
}

// function that fills a buffer with the string representation of a number
fn u16_to_str(n: u16, buf: &mut [u8; 5]) {
    let mut n = n;
    for i in (0..5).rev() {
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
}

// function that prints a u16 number and can add a blank space after it or not or a new line
fn print_u16_with_space_or_new_line(
    tx: &mut Tx<USART1>,
    n: u16,
    add_space: bool,
    add_new_line: bool,
) {
    let mut s: [u8; 5] = [0; 5];
    u16_to_str(n, &mut s);
    let _ = s.iter().map(|c| block!(tx.write(*c))).last();
    if add_space {
        let _ = block!(tx.write(b' '));
    }
    if add_new_line {
        let _ = block!(tx.write(b'\r'));
        let _ = block!(tx.write(b'\n'));
    }
}

// function that prints an array of u16 numbers highlighting the two maximum values and the two minimum values.
// The maximum values are printed in red and the minimum values are printed in green.
fn print_u16_array_with_max_and_min(tx: &mut Tx<USART1>, readings: [u16; 8]) {
    let mut max1 = 0;
    let mut max2 = 0;
    let mut min1 = u16::MAX;
    let mut min2 = u16::MAX;
    let mut max1_index = 0;
    let mut max2_index = 0;
    let mut min1_index = 0;
    let mut min2_index = 0;
    for (i, &reading) in readings.iter().enumerate() {
        if reading > max1 {
            max2 = max1;
            max1 = reading;
            max2_index = max1_index;
            max1_index = i;
        } else if reading > max2 {
            max2 = reading;
            max2_index = i;
        }
        if reading < min1 {
            min2 = min1;
            min1 = reading;
            min2_index = min1_index;
            min1_index = i;
        } else if reading < min2 {
            min2 = reading;
            min2_index = i;
        }
    }
    for (i, &reading) in readings.iter().enumerate() {
        if i == max1_index || i == max2_index {
            let _ = block!(tx.write(b'\x1B'));
            let _ = block!(tx.write(b'['));
            let _ = block!(tx.write(b'3'));
            let _ = block!(tx.write(b'1'));
            let _ = block!(tx.write(b'm'));
            print_u16_with_space_or_new_line(tx, reading, true, false);
            let _ = block!(tx.write(b'\x1B'));
            let _ = block!(tx.write(b'['));
            let _ = block!(tx.write(b'0'));
            let _ = block!(tx.write(b'm'));
        } else if i == min1_index || i == min2_index {
            let _ = block!(tx.write(b'\x1B'));
            let _ = block!(tx.write(b'['));
            let _ = block!(tx.write(b'3'));
            let _ = block!(tx.write(b'2'));
            let _ = block!(tx.write(b'm'));
            print_u16_with_space_or_new_line(tx, reading, true, false);
            let _ = block!(tx.write(b'\x1B'));
            let _ = block!(tx.write(b'['));
            let _ = block!(tx.write(b'0'));
            let _ = block!(tx.write(b'm'));
        } else {
            print_u16_with_space_or_new_line(tx, reading, true, false);
        }
    }
}
