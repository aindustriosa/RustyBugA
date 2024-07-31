// This crate is a library to manage the Quadrature Encoder interface of STM32F1xx MCUs
// The timers supported are TIM2 to TIM5 (General-purpose timers) and TIM1 and TIM8 (Advanced-control timers).
// Each timer has 4 channels, so the library supports up to 2 encoders by timer since a quadrature encoder uses 2 channels.

#![no_std]

// re-export the HAL
pub use hal_encoder::EncoderController;

// General-purpose timers (TIM2 to TIM5)
pub mod tim2_to_tim5;

// Advanced-control timers (TIM1 and TIM8)
// to-do