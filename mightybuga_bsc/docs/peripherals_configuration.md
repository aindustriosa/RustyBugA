# Peripherals configuration
Here are some notes regarding the MCU configuration.

## Motors
Motors use the Timer 1 for PWM based velocity control. The channels used are:
 - Channel 1 for left motor PA11.
 - Channel 4 for right motor PA08.

There are used 2 extra GPIOs per motor for driver control:
- Right motor:
 - In1 PB09
 - In2 PB08
- Left motor:
 - In1 PB05
 - In2 PA12

## Encoders
Timer 4