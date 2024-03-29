#![no_std]
#![allow(non_camel_case_types)]

// reexport hal crates to allow users to directly refer to them
// like in https://github.com/therealprof/nucleo-f103rb/blob/master/src/lib.rs
pub use stm32f1xx_hal as hal;

use hal::pac::*;
use hal::prelude::*;
use hal::serial::*;
use hal::timer::SysDelay;

use engine::engine::Engine;
use engine::motor::Motor;
use stm32f1xx_hal::timer::PwmChannel;

pub use crate::hal::*;

pub mod timer_based_buzzer;
use timer_based_buzzer::TimerBasedBuzzer;

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
    // delay provider
    pub delay: SysDelay,
    // UART
    pub uart: UART,
    // LEDs
    pub leds: Leds,
    // Buzzer
    pub buzzer: TimerBasedBuzzer,
    // Engine
    pub engine: Engine<
        Motor<
            gpio::Pin<'B', 5, gpio::Output>,
            gpio::Pin<'A', 12, gpio::Output>,
            PwmChannel<TIM1, 0>,
        >,
        Motor<
            gpio::Pin<'B', 9, gpio::Output>,
            gpio::Pin<'B', 8, gpio::Output>,
            PwmChannel<TIM1, 3>,
        >,
    >,
}

impl Mightybuga_BSC {
    pub fn take() -> Result<Self, ()> {
        let dp = hal::pac::Peripherals::take().ok_or(())?;
        // Take ownership over the raw flash and rcc devices and convert them into the corresponding
        // HAL structs
        let mut flash = dp.FLASH.constrain();

        // We need to enable the clocks here for the peripherals we want to use because the
        // `constrain` frees the `RCC` register proxy.

        // Enable the timer 3 clock in the RCC register (we net to do this before the constrain)
        dp.RCC.apb1enr.modify(|_, w| w.tim3en().set_bit());

        let rcc = dp.RCC.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(72.MHz())
            .freeze(&mut flash.acr);

        let cp = cortex_m::Peripherals::take().unwrap();
        let mut delay = cp.SYST.delay(&clocks);
        delay.delay(300.millis());

        let mut afio = dp.AFIO.constrain();

        // Serial port configuration
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

        // LED configuration
        let mut gpioc = dp.GPIOC.split();
        let d1: gpio::Pin<'C', 13, gpio::Output> = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        let mut gpiob = dp.GPIOB.split();

        // Engine/Motors configuration (PWM)
        let pwm_motor_pins = (
            gpioa.pa8.into_alternate_push_pull(&mut gpioa.crh),
            gpioa.pa11.into_alternate_push_pull(&mut gpioa.crh),
        );

        // We set the PWM frequency to 9 kHz so that the motor doesn't make a lot of noise.
        let pwm = dp
            .TIM1
            .pwm_hz(pwm_motor_pins, &mut afio.mapr, 9.kHz(), &clocks)
            .split();

        let mut left_motor_channel = pwm.0;
        let mut right_motor_channel = pwm.1;

        left_motor_channel.enable();
        right_motor_channel.enable();

        let motor_left = Motor::new(
            gpiob.pb5.into_push_pull_output(&mut gpiob.crl),
            gpioa.pa12.into_push_pull_output(&mut gpioa.crh),
            left_motor_channel,
        );
        let motor_right = Motor::new(
            gpiob.pb9.into_push_pull_output(&mut gpiob.crh),
            gpiob.pb8.into_push_pull_output(&mut gpiob.crh),
            right_motor_channel,
        );

        // Engine is the struct which contains all the logics regarding the motors
        let engine = Engine::new(motor_left, motor_right);

        // Buzzer configuration
        let (_pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);
        // Remap TIM3 gpio pin
        afio.mapr
            .modify_mapr(|_, w| unsafe { w.tim3_remap().bits(0b10) });
        let buzzer_pin = pb4.into_alternate_push_pull(&mut gpiob.crl);
        let buzzer = TimerBasedBuzzer::new(dp.TIM3, buzzer_pin);

        // Return the initialized struct

        Ok(Mightybuga_BSC {
            delay,
            uart: UART { rx, tx },
            leds: Leds { d1 },
            buzzer,
            engine,
        })
    }
}
