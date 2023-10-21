# Articles

## 2020_07 Rust on STM32: Getting started
https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/
Usa Bluepill y STLink
## 2021_02 Rust and STM32: A Quick Start Guide
https://bacelarhenrique.me/2021/02/21/rust-and-stm32-a-quick-start-guide.html
usa una NUCLEO-F103RB y STLink
## 2022_11 Embedded Rust & Embassy: GPIO Button Controlled Blinking
https://dev.to/apollolabsbin/embedded-rust-embassy-gpio-button-controlled-blinking-3ee6
   This blog post is the first one of a multi-part series of posts where I will explore various peripherals of the STM32 microcontroller using the embedded Rust embassy framework.
   I will be working with the STM32F401


## 2023_08 What the HAL? The Quest for Finding a Suitable Embedded Rust HAL
https://dev.to/apollolabsbin/what-the-hal-the-quest-for-finding-a-suitable-embedded-rust-hal-2i02

## 2021_01 [Programming a Blue/Black Pill STM32 Board with a ST-Link v2](https://waterpigs.co.uk/articles/black-blue-pill-stm32-st-link-connection/)
   While trying to learn embedded rust, I ran up against issues connecting my black pill STM32F103 boards to the ST-Link v2. The solution is simple but badly documented, so here it is with nice illustrations.

## 2020_10 Using Rust for a simple hardware project
https://blog.tonari.no/rust-simple-hardware-project
   We actually started with the STM32F411 (the board with USB C) and later moved to to the STM32F103 because they were easier to acquire and both boards covered our hardware needs. We essentially followed [this guide](https://www.gregwoods.co.uk/microcontroller/embedded/rust/stm32/blackpill/bluepill/convert-stm32f1-blink-to-stm32f4/) in reverse, though it will probably become outdated over time.
   Here's the [diff](https://github.com/tonarino/panel-firmware/pull/2) which moved us from the STM32F411 to the STM32F103.
   
https://github.com/tonarino/panel-firmware
   The current firmware uses this model: STM32F411RE
   This firmware can also be debugged on a USB-C "black pill" board, linked here: [Board Info](https://stm32-base.org/boards/STM32F411CEU6-WeAct-Black-Pill-V2.0)
   The firmware also used to run on the cheaper STM32F103-based boards. Look in the commit history for working with that. It might be beneficial in the future to support both simultaneously and enable one or the other via feature flags.
## 2022_05 Building a sailing starter board with Rust (RTIC)
https://gill.net.in/posts/stm32-pcb-sailing-and-rust/
   The MCU I chose was [STM32f103C6](https://www.st.com/content/st_com/en/products/microcontrollers-microprocessors/stm32-32-bit-arm-cortex-mcus/stm32-mainstream-mcus/stm32f1-series/stm32f103/stm32f103c6.html) its small and an overkill for our use case, however I managed to secure a bunch of them a few months earlier. With 32KB flash and 10KB ram
   I created a prototype on a breadboard using a [STM32 Blue Pill](https://stm32-base.org/boards/STM32F103C8T6-Blue-Pill.html)
   RTIC is a rust framework for embedded devices, I have only used it on ARM chips, not sure if it works properly on other platforms. It allows you to structure your program based upon time based and other interrupts to perform certain tasks.
   Here though we are going to only use time based interrupts, since that’s all we really need to build a timer. Before we dig into the code for this project, let’s have quick look at some of the key concepts of RTIC.

1. Everything is defined within a module annotated with `#[rtic::app(...)]`, The app attribute has a mandatory device argument that takes a path as a value.
    
2. A Task is just a function that performs, well a task, and can use shared or local variables. There can be arbitrary number of tasks, with at least one that is required with annotate with `#[init]` attribute. `init` task must have `fn(init::Context) -> (Shared, Local, init::Monotonics)` signature and is used to initialize hardware and local/shared state. `init` is only called once after the system reset. There is also a `idle` task with `fn(idle::Context) -> !` signature, that is auto defined unless explicitly, idle loops forever much like Arduino `loop`, you are not required to use it or it can just be used to put the CPU to sleep when there is nothing else to do.
    
3. RTIC has two types of state, `Shared` and `Local`, as the name implies `Local` state can be used by one and only one task and will be available to it via the `Context`, whereas multiple tasks can used shared state via the `Context` as well but it implements a Mutex trait so only one task at a time can have access to its internal data. Additionally if you don’t need to initialize state with something with the [HAL](https://github.com/stm32-rs/stm32f1xx-hal) , you can define local state at the task level with annotation.

   I use following tools to work with stm32 arm based devices, mainly from ferrous-systems' [Knurling project tools](https://knurling.ferrous-systems.com/tools/) . If you are not familiar, `knurling` gives you grip on bare metal (I love this fun name).

- [probe-run](https://crates.io/crates/probe-run) , A custom Cargo runner that transparently runs Rust firmware on an embedded device.
- [cargo-flash](https://crates.io/crates/cargo-flash) , Provides a cargo subcommand to flash ELF binaries onto ARM chips.
- [flip-link](https://crates.io/crates/flip-link) , Adds zero-cost stack overflow protection to your embedded programs.

https://github.com/mygnu/rregatta-firmware

# Base libs repos

##  [`stm32f1xx-hal`](https://github.com/stm32-rs/stm32f1xx-hal#stm32f1xx-hal)
https://crates.io/crates/stm32f1xx-hal
https://crates.io/crates/stm32f1
citado en https://rust-classes.com/chapter_embedded.html

Ejemplos de periféricos:
https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples

## https://crates.io/crates/cortex-m-log

##  [`alt-stm32f30x-hal`](https://github.com/copterust/alt-stm32f30x-hal#alt-stm32f30x-hal)

## https://crates.io/crates/async-stm32f1xx


##  [nucleo-f103rb](https://github.com/therealprof/nucleo-f103rb/tree/master#nucleo-f103rb)


## https://docs.embassy.dev/embassy-stm32/git/stm32f103c8/index.html


## https://github.com/knurling-rs/app-template
Quickly set up a [`probe-run`](https://crates.io/crates/probe-run) + [`defmt`](https://github.com/knurling-rs/defmt) + [`flip-link`](https://github.com/knurling-rs/flip-link) embedded project

## RTIC: The hardware accelerated Rust RTOS
https://github.com/rtic-rs/rtic

https://rtic.rs/2/book/en/
Real-Time Interrupt-driven Concurrency (RTIC) framework for ARM Cortex-M microcontrollers.

RTIC aims to provide the lowest level of abstraction needed for developing robust and reliable embedded software.

It provides a minimal set of required mechanisms for safe sharing of mutable resources among interrupts and asynchronously executing tasks. The scheduling primitives leverages on the underlying hardware for unparalleled performance and predictability, in effect RTIC provides in Rust terms a zero-cost abstraction to concurrent real-time programming.
# Repos con proyectos base

## https://cgit.pinealservo.com/BluePill_Rust/blue_pill_base (mar 2020)

## [stm32-blue-pill-rust](https://github.com/lupyuen/stm32-blue-pill-rust#stm32-blue-pill-rust)(Dec 2020)
https://github.com/lupyuen/stm32-blue-pill-rust

Anticuado

## STM32_blink (2019)
https://gitlab.com/jounathaen/stm32_blink#stm32_blink

## https://github.com/geomatsi/rust-blue-pill-tests/tree/master (2022)
usa probe-rs

## https://github.com/piedoom/blue-pill-quickstart-rtic (2020)

## [STM32 BluePill in Rust - Project template](https://github.com/joaocarvalhoopen/stm32_bluepill_in_rust__Template#stm32-bluepill-in-rust---project-template) (2022)

# Rust con blue pill Rust projects
## https://github.com/Windfisch/midikraken/tree/master/firmware
An open-source, open-hardware MIDI-USB-interface supporting up to 16 (and maybe beyond?) MIDI ports. The hardware is based on a STM32F103 _"blue pill"_ board which can be cheaply sourced from your favourite chinese seller ([note about quality differences](https://github.com/Windfisch/analog-synth/blob/master/bluepill.md)), plus other garden variety components such as HC2630 opto couplers (that's just two 6N137 in one package), shift registers, some LEDs and resistors.

## https://github.com/anglerud/lps25_pressure_sensor_demo
A barometer based on Adafruit's [LPS25 breakout board](https://www.adafruit.com/product/4530), written in Rust, running on the Blue Pill board and displaying the result on an HD44780 LCD panel.

## https://github.com/bmc-labs/nerdclock
A [BluePill board](https://stm32-base.org/boards/STM32F103C8T6-Blue-Pill.html) with the CAN wired up is used here. Everything you'll need should be documented in this repo here.

- Our friend's existing prototype
- The BluePill board with CAN wired up
- The following software:
    - [stm32f1xx-hal](https://github.com/stm32-rs/stm32f1xx-hal), which is a Rust implementation of the core functionality for the STM32F1 series
    - The [knurling-rs app template](https://github.com/knurling-rs/app-template) developed by [the knurling-rs project](https://knurling.ferrous-systems.com/) for bare-metal Rust

## [pomia-rs](https://github.com/VersBinarii/pomia-rs#pomia-rs)
Were testing here how well Rust works for an embedded project
So far we have up and running:
- [RTIC](https://github.com/rtic-rs/cortex-m-rtic)
- Timer3 interrupt
- PWM used for generating music
- SPI for driving 128x160 LCD display
- Some basic graphics based on [embedded_graphics](https://github.com/embedded-graphics/embedded-graphics)
- Basic UI allowing changing views and basic edit mode.
- I2C based temperature/humidity/pressure sensor BME280
- EXTI interrupt based button handling
- RTC based clock

# [Youtube video](https://github.com/VersBinarii/pomia-rs#youtube-video)

There is a bunch of videos on youtube showing progress in implementing the above functionality. You can find it [here](https://www.youtube.com/watch?v=Meqhiogdp1o)

# Others

## https://crates.io/crates/mpu9250
