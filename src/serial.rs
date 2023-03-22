
extern "C" {
    pub fn serial_init(baud: i32);
    pub fn serial_putc(ns: i32);
    pub fn serial_flush();
    pub fn report(str: &str);
}

/// Responsible for handling all serial communication
pub struct Serial { }

impl Serial {
    /// Initialize the serial interface with the desired baud rate.
    pub fn init(baud: i32) {
        unsafe {
            serial_init(baud);
        }
    }

    /// Sends a single character to the serial port. A call to `Serial::flush()` may be
    /// required to actually write the data.
    pub fn putc(c: i32) {
        unsafe {
            serial_putc(c);
        }
    }

    /// Flushes any outstanding data out to the serial port.
    pub fn flush() {
        unsafe {
            serial_flush();
        }
    }

    /// Writes a string out to the serial port. A call to `Serial::flush()` is not required.
    pub fn report(s: &str) {
        unsafe {
            report(s);
        }
    }
}