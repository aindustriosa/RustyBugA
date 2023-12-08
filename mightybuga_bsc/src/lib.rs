#![no_std]
#![allow(non_camel_case_types)]

pub use stm32f1xx_hal as hal;

pub use crate::hal::pac::interrupt::*;
pub use crate::hal::pac::*;
pub use crate::hal::prelude::*;
pub use crate::hal::*;
pub use cortex_m::*;
pub use cortex_m_rt::*;
