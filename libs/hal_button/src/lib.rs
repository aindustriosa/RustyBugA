#![no_std]

use embedded_hal::digital::v2::InputPin;

pub trait ButtonController {
    // This function returns true if the button is on pressed state
    fn is_pressed(&self) -> bool;

    // This function returns true if the button changed its state since the last time this function was called
    fn is_changed(&mut self) -> bool;
}

// This struct handles the resources and state regarding a button.
// Set PulledUp to true if the button has a pull-up resistor, false otherwise. It is
// defined as const generic to optimize the code of inverted logic at compile time.
pub struct Button<Pin: InputPin, const PULLED_UP: bool> {
    pin: Pin,
    last_state: bool,
}

// Implementation of the Button struct based on the ButtonController and InputPin traits
impl<Pin: InputPin, const PULLED_UP: bool> Button<Pin, PULLED_UP> {
    pub fn new(pin: Pin) -> Self {
        Button {
            pin,
            last_state: false,
        }
    }
}

// Implementation of the ButtonController trait for the Button struct
impl<const PULLED_UP: bool, Pin: InputPin> ButtonController for Button<Pin, PULLED_UP> {
    fn is_pressed(&self) -> bool {
        // this is resolved at compile time (zero cost abstraction!)
        if PULLED_UP {
            self.pin.is_low().ok().unwrap()
        } else {
            self.pin.is_high().ok().unwrap()
        }
    }

    fn is_changed(&mut self) -> bool {
        let current_state = self.is_pressed();
        let changed = current_state != self.last_state;
        self.last_state = current_state;
        changed
    }
}
