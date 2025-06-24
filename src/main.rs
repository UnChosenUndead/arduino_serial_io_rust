#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;
const MAX_LEN: usize = 20; // Maximum length of the string

struct MyString {
    data: [u8; MAX_LEN],
    length: usize,
}

impl MyString {
    fn new() -> Self {
        MyString {
            data: [0; MAX_LEN],
            length: 0,
        }
    }

    fn add_char(&mut self, c: char) {
        if self.length < MAX_LEN {
            self.data[self.length] = c as u8; // Convert char to u8
            self.length += 1;
        }
    }

    fn as_str(&self) -> &str {
        // Convert the internal byte array to a string slice
        core::str::from_utf8(&self.data[..self.length]).unwrap()
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut led = pins.d6.into_output();
    ufmt::uwriteln!(&mut serial, "Hello, Rust!\r\n").unwrap();

    loop {
        let mut input: MyString = MyString::new();
        led.toggle();
        while let byte = serial.read_byte() {
            // Используем read_byte
            match byte {
                b'\n' => break,
                _ => {
                    input.add_char(byte as char);
                }
            }
        }

        serial.write_str("You entered: ").unwrap();
        serial.write_str(input.as_str()).unwrap();
        serial.write_str("\r\n").unwrap();
        arduino_hal::delay_ms(1000);
    }
}
