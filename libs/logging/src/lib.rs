// A simple logging library for embedded Rust
#![no_std]
extern crate alloc;

use core::convert::Infallible;

use embedded_hal::blocking::serial::Write;
use nb::block;


pub fn log(tx: &mut dyn Write<u8, Error = Infallible>, s: &str) {
    for c in s.chars() {
        block!(match tx.bwrite_all(&[c as u8]) {
            Ok(_) => Ok(()),
            Err(_) => Err(nb::Error::Other(())),
        })
        .unwrap();
    }
}

pub fn log_u16(tx: &mut dyn Write<u8, Error = Infallible>, val: &u16) {
    for digit_index in (1..6).rev() {

        // We have to temporarily the u16 to u32, otherwise doing 10^5 would cause an
        // overflow in a u16 (100,000 > 65,535)
        let digit = (*val as u32 % 10u32.pow(digit_index as u32)) / 10u32.pow(digit_index as u32 - 1);
        let digit_character: u8 = digit as u8 + 48; // 48 is the offset of the numbers in the ASCII table

        block!(match tx.bwrite_all(&[digit_character]) {
            Ok(_) => Ok(()),
            Err(_) => Err(nb::Error::Other(())),
        })
        .unwrap();
    }
}

pub fn log_u16_array(tx: &mut dyn Write<u8, Error = Infallible>, array: &[u16]) {
    log(tx,"[");

    array.iter().enumerate().for_each(|(index, &value)| {
        log_u16(tx, &value);

        if index != array.len() - 1 {
            log(tx,", ");
        }
    });

    log(tx,"]");
}


#[cfg(test)]
mod tests {
    use alloc::string::String;

    use core::convert::Infallible;

    use crate::{log, log_u16, log_u16_array};

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
    fn test_logger() {
        let mut mock_writer = MockWriter::new();
        log(& mut mock_writer, "Hello");
        assert_eq!(mock_writer.get_string(), "Hello");
    }

    #[test]
    fn test_log_u16() {
        let mut mock_writer = MockWriter::new();
        log_u16(&mut mock_writer, &3456u16);
        assert_eq!(mock_writer.get_string(), "03456");
    }

    #[test]
    fn test_log_u16_array() {
        let mut mock_writer = MockWriter::new();
        log_u16_array(&mut mock_writer, &[100u16, 250u16, 500u16]);
        assert_eq!(mock_writer.get_string(), "[00100, 00250, 00500]");
    }
}
