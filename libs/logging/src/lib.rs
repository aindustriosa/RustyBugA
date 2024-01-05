// A simple logging library for embedded Rust

use embedded_hal::blocking::serial::Write;
use nb::block;

pub struct Logger<'a> {
    pub uart: &'a mut dyn Write<u8, Error = nb::Error<()>>,
}

impl<'a> Logger<'a> {
    pub fn new(uart: &'a mut dyn Write<u8, Error = nb::Error<()>>) -> Self {
        Logger { uart }
    }

    pub fn log(&mut self, s: &str) {
        for c in s.chars() {
            block!(self.uart.bwrite_all(&[c as u8])).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Logger;

    // mock of embedded_hal::blocking::serial::Write that just writes to a string
    struct MockWriter {
        s: String,
    }

    impl embedded_hal::blocking::serial::Write<u8> for MockWriter {
        type Error = nb::Error<()>;

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
        println!("Hi 42!");
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
