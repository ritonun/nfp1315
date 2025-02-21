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

    /// Send a command
    fn send_command(&mut self, command: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x00, command])?;
        Ok(())
    }

    /// Send a data into the GDDRAM
    fn send_data(&mut self, data: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x40, data])?;
        Ok(())
    }

    pub fn init_display(&mut self) -> Result<(), E> {
        // turn the display off
        self.send_command(0xAE)?;

        // set mux ratio for a 128x64 display
        self.send_command(0xA8)?;
        self.send_command(0x3F)?;

        // set display offset
        self.send_command(0xD3)?;
        self.send_command(0x00)?;

        // set display start line
        self.send_command(0x40)?;

        // set segment remap
        self.send_command(0xA1)?;

        // set com pins output direction
        self.send_command(0xC8)?;

        // set com pins hardware configurations
        self.send_command(0xDA)?;
        self.send_command(0x12)?;

        // set contrast control
        self.send_command(0x81)?;
        self.send_command(0x7F)?;

        // disable entire display on
        self.send_command(0x4A)?;

        // set normal display
        self.send_command(0xA6)?;

        // set osc. frequ
        self.send_command(0xD5)?;
        self.send_command(0x80)?;

        // enable charge pump regulator
        self.send_command(0x8D)?;
        self.send_command(0x14)?;

        // turn display on
        self.send_command(0xAF)?;

        Ok(())
    }
}
