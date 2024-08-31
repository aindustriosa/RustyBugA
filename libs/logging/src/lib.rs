// A simple logging library for embedded Rust
#![no_std]
extern crate alloc;

use core::convert::Infallible;

use embedded_hal::blocking::serial::Write;
use nb::block;

pub struct Logger<'a> {
    pub uart: &'a mut dyn Write<u8, Error = Infallible>,
}

impl<'a> Logger<'a> {
    pub fn new(uart: &'a mut dyn Write<u8, Error = Infallible>) -> Self {
        Logger { uart }
    }

    pub fn log(&mut self, s: &str) {
        for c in s.chars() {
            block!(match self.uart.bwrite_all(&[c as u8]) {
                Ok(_) => Ok(()),
                Err(_) => Err(nb::Error::Other(())),
            })
            .unwrap();
        }
    }

    pub fn log_u16(&mut self, val: &u16) {
        for digit_index in (1..6).rev() {

            // We have to temporarily convert the u16 to u32, otherwise doing 10⁶5 would cause an
            // overflow in a u16 (100,000 > 65,535)
            let digit = ((*val as u32 % 10u32.pow(digit_index as u32)) / 10u32.pow(digit_index as u32 - 1)) as u16;
            let digit_character = digit + 48; // 48 is the offset of the numbers in the ASCII table

            block!(match self.uart.bwrite_all(&[digit_character as u8]) {
                Ok(_) => Ok(()),
                Err(_) => Err(nb::Error::Other(())),
            })
            .unwrap();
        }
    }

    pub fn log_u16_array(&mut self, array: &[u16]) {
        self.log("[");

        array.iter().enumerate().for_each(|(index, &value)| {
            self.log_u16(&value);

            if index != array.len() - 1 {
                self.log(", ");
            }
        });

        self.log("]");
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use core::convert::Infallible;

    use crate::Logger;

    // mock of embedded_hal::blocking::serial::Write that just writes to a char vector
    struct MockWriter {
        s: String,
    }

    impl embedded_hal::blocking::serial::Write<u8> for MockWriter {
        type Error = Infallible;

        fn bwrite_all(&mut self, s: &[u8]) -> Result<(), Self::Error> {
            for c in s {
                self.s.push(*c as char);
            }
            Ok(())
        }

        fn bflush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { s: String::new() }
        }

        fn get_string(&self) -> &str {
            &self.s
        }
    }

    #[test]
    fn test_simple() {
        // Just a simple test to make sure the test framework is working
        assert_eq!(true, true);
    }

    #[test]
    fn test_logger() {
        let mut mock_writer = MockWriter::new();
        let mut logger = Logger::new(&mut mock_writer);
        logger.log("Hello");
        assert_eq!(mock_writer.get_string(), "Hello");
    }
}
