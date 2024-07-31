#![no_std]
#![allow(non_camel_case_types)]
#![allow(static_mut_refs)]

// reexport hal crates to allow users to directly refer to them
// like in https://github.com/therealprof/nucleo-f103rb/blob/master/src/lib.rs
pub use stm32f1xx_hal as hal;

use hal::adc::Adc;
use hal::gpio::PullDown;
use hal::pac::*;
use hal::prelude::*;
use hal::serial::*;
use hal::timer::SysDelay;

use core::cell::RefCell;

use heapless::{
    arc_pool,
    pool::arc::ArcBlock,
};
use core::ops::Deref;

use engine::engine::Engine;
use engine::motor::Motor;
use stm32f1xx_hal::timer::PwmChannel;

mod light_sensor_array;
use light_sensor_array::LightSensorArray;

mod battery_sensor;
use battery_sensor::BatterySensor;

pub use crate::hal::*;

pub mod timer_based_buzzer;
use timer_based_buzzer::TimerBasedBuzzer;

pub use hal_encoder_stm32f1xx::tim2_to_tim5::*;

pub mod prelude {
    pub use cortex_m_rt::entry;
    pub use stm32f1xx_hal::prelude::{
        _embedded_hal_blocking_delay_DelayMs, _embedded_hal_blocking_delay_DelayUs, _fugit_ExtU32,
    };
}

// Implement ArcPool for the ADC1
arc_pool!(ADC_POOL: RefCell<Adc<ADC1>>);

pub struct Mightybuga_BSC {
    // LEDs
    pub led_d1: gpio::Pin<'C', 13, gpio::Output>,
    pub led_d2: gpio::Pin<'B', 12, gpio::Output>,
    // UART
    pub serial: Serial<
        USART1,
        (
            gpio::Pin<'A', 9, gpio::Alternate>,
            gpio::Pin<'A', 10, gpio::Input>,
        ),
    >,
    // delay provider
    pub delay: SysDelay,
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
    // Buttons
    pub btn_1: hal_button::Button<gpio::Pin<'B', 13, gpio::Input<PullDown>>, false>,
    pub btn_2: hal_button::Button<gpio::Pin<'C', 15, gpio::Input<PullDown>>, false>,
    pub btn_3: hal_button::Button<gpio::Pin<'C', 14, gpio::Input<PullDown>>, false>,
    // Encoders
    pub encoder_r: IncrementalEncoder,
    pub encoder_l: IncrementalEncoder,
    // Light sensor array
    pub light_sensor_array: LightSensorArray,
    // Battery sensor
    pub battery_sensor: BatterySensor,
}

impl Mightybuga_BSC {
    pub fn take() -> Result<Self, ()> {
        let dp = hal::pac::Peripherals::take().ok_or(())?;
        // Take ownership over the raw flash and rcc devices and convert them into the corresponding
        // HAL structs
        let mut flash = dp.FLASH.constrain();

        // reset and clock control
        let rcc = dp.RCC;
        rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());
        rcc.apb1enr.modify(|_, w| w.tim3en().set_bit());
        rcc.apb1enr.modify(|_, w| w.tim4en().set_bit());

        // Use external crystal for clock
        let clocks = rcc
            .constrain()
            .cfgr
            .use_hse(8.MHz())
            .sysclk(72.MHz())
            .freeze(&mut flash.acr);

        let cp = cortex_m::Peripherals::take().unwrap();
        let mut delay = cp.SYST.delay(&clocks);
        delay.delay(300.millis());

        // GPIO ports
        let mut gpioa = dp.GPIOA.split();
        let mut gpiob = dp.GPIOB.split();
        let mut gpioc = dp.GPIOC.split();

        // Alternate function I/O remapping
        let mut afio = dp.AFIO.constrain();
        let (_pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        // LEDs configuration
        let d1 = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
        let d2 = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

        // Serial port configuration
        let serial_uart = Serial::new(
            dp.USART1,
            (
                gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),
                gpioa.pa10.into_floating_input(&mut gpioa.crh),
            ),
            &mut afio.mapr,
            Config::default().baudrate(115_200.bps()),
            &clocks,
        );

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
        afio.mapr
            .modify_mapr(|_, w| unsafe { w.tim3_remap().bits(0b10) });
        let buzzer_pin = pb4.into_alternate_push_pull(&mut gpiob.crl);
        let buzzer = TimerBasedBuzzer::new(dp.TIM3, buzzer_pin);

        // Button configurations
        let btn_1 = hal_button::Button::new(gpiob.pb13.into_pull_down_input(&mut gpiob.crh));
        let btn_2 = hal_button::Button::new(gpioc.pc15.into_pull_down_input(&mut gpioc.crh));
        let btn_3 = hal_button::Button::new(gpioc.pc14.into_pull_down_input(&mut gpioc.crh));

        // Encoder right
        let encoder_r = IncrementalEncoder::new(
            dp.TIM4.deref(),
            TimerChannels::Ch1Ch2,
            EncoderPolarity::PolarityBA,
        );

        // Encoder left
        afio.mapr
            .modify_mapr(|_, w| unsafe { w.tim2_remap().bits(0b01) });
        let encoder_l = IncrementalEncoder::new(
            dp.TIM2.deref(),
            TimerChannels::Ch1Ch2,
            EncoderPolarity::PolarityBA,
        );

        // Generate the memory block in which the adc will be allocated
        let adc_arc_block: &'static mut ArcBlock<RefCell<Adc<ADC1>>> = unsafe {
            static mut B: ArcBlock<RefCell<Adc<ADC1>>> = ArcBlock::new();
            &mut B
        };
        ADC_POOL.manage(adc_arc_block);

        // Allocate the Adc in an Arc to share it
        let adc_arc = match ADC_POOL.alloc(RefCell::new(Adc::adc1(dp.ADC1, clocks))) {
            Ok(adc_arc) => adc_arc,
            Err(_) => panic!("Couldn't get the adc arc"),
        };

        let light_sensor_array = LightSensorArray {
            led: gpiob.pb1.into_push_pull_output(&mut gpiob.crl),
            sensor_0: gpioa.pa0.into_analog(&mut gpioa.crl),
            sensor_1: gpioa.pa1.into_analog(&mut gpioa.crl),
            sensor_2: gpioa.pa2.into_analog(&mut gpioa.crl),
            sensor_3: gpioa.pa3.into_analog(&mut gpioa.crl),
            sensor_4: gpioa.pa4.into_analog(&mut gpioa.crl),
            sensor_5: gpioa.pa5.into_analog(&mut gpioa.crl),
            sensor_6: gpioa.pa6.into_analog(&mut gpioa.crl),
            sensor_7: gpioa.pa7.into_analog(&mut gpioa.crl),
            adc: adc_arc.clone(),
        };

        let battery_sensor = BatterySensor {
            sensor_0: gpiob.pb0.into_analog(&mut gpiob.crl),
            adc: adc_arc.clone(),
        };

        // Return the initialized struct
        Ok(Mightybuga_BSC {
            led_d1: d1,
            led_d2: d2,
            serial: serial_uart,
            delay,
            buzzer,
            engine,
            encoder_r,
            encoder_l,
            btn_1,
            btn_2,
            btn_3,
            light_sensor_array,
            battery_sensor,
        })
    }
}
