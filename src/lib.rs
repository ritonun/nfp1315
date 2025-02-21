#![no_std]

use embedded_hal::i2c::{Error, I2c};

pub struct SSD1306<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C, E> SSD1306<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    fn send_command(&mut self, command: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x00, command])?;
        Ok(())
    }
}
