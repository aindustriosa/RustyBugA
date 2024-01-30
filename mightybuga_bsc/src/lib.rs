#![no_std]
#![allow(non_camel_case_types)]

// reexport hal crates to allow users to directly refer to them
// like in https://github.com/therealprof/nucleo-f103rb/blob/master/src/lib.rs
pub use stm32f1xx_hal as hal;

use hal::pac::*;
use hal::prelude::*;
use hal::serial::*;
use hal::timer::SysDelay;
use hal::timer::Tim2NoRemap;

use engine::engine::Engine;
use engine::motor::Motor;

pub use crate::hal::*;

pub mod prelude {
    pub use cortex_m_rt::entry;
    pub use stm32f1xx_hal::prelude::{
        _embedded_hal_blocking_delay_DelayMs, _embedded_hal_blocking_delay_DelayUs, _fugit_ExtU32,
    };
}

pub struct UART {
    pub rx: Rx<USART1>,
    pub tx: Tx<USART1>,
}
pub struct Leds {
    pub d1: gpio::Pin<'C', 13, gpio::Output>,
}

pub struct Mightybuga_BSC {
    pub delay: SysDelay,
    pub uart: UART,
    pub leds: Leds,
}

impl Mightybuga_BSC {
    pub fn take() -> Result<Self, ()> {
        let dp = hal::pac::Peripherals::take().ok_or(())?;
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);

        let cp = cortex_m::Peripherals::take().unwrap();
        let mut delay = cp.SYST.delay(&clocks);
        delay.delay(300.millis());

        let mut afio = dp.AFIO.constrain();
        let mut gpioa = dp.GPIOA.split();
        let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
        let rx = gpioa.pa10;
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
        let (tx, rx) = serial.split();

        let mut gpioc = dp.GPIOC.split();
        let d1: gpio::Pin<'C', 13, gpio::Output> = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        let mut gpiob = dp.GPIOB.split();

        let pwm_motor_pins = (
            gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl),
            gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl),
        );

        let pwm = dp
            .TIM2
            .pwm_hz::<Tim2NoRemap, _, _>(pwm_motor_pins, &mut afio.mapr, 1.kHz(), &clocks)
            .split();

        let left_motor_channel = pwm.0;
        let right_motor_channel = pwm.1;

        let motor_left = Motor::new(
            gpiob.pb10.into_push_pull_output(&mut gpiob.crh),
            gpiob.pb12.into_push_pull_output(&mut gpiob.crh),
            left_motor_channel,
        );
        let motor_right = Motor::new(
            gpiob.pb9.into_push_pull_output(&mut gpiob.crh),
            gpiob.pb8.into_push_pull_output(&mut gpiob.crh),
            right_motor_channel,
        );

        // Engine is the struct which contains all the logics regarding the motors
        let mut engine = Engine::new(motor_left, motor_right);
        // This is to validate that everything is in place
        engine.forward(10);
        engine.backward(100);
        engine.left(10, 5);
        engine.right(10, 5);
        engine.stop();

        Ok(Mightybuga_BSC {
            delay,
            uart: UART { rx, tx },
            leds: Leds { d1 },
        })
    }
}
