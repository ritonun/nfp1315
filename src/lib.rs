#![no_std]

use embedded_hal::i2c::{Error, I2c};

const FONT_5X8: [[u8; 5]; 27] = [
    [0x00, 0x00, 0x00, 0x00, 0x00], // Space
    [0x7E, 0x11, 0x11, 0x11, 0x7E], // A
    [0x7F, 0x49, 0x49, 0x49, 0x36], // B
    [0x3E, 0x41, 0x41, 0x41, 0x22], // C
    [0x7F, 0x41, 0x41, 0x22, 0x1C], // D
    [0x7F, 0x49, 0x49, 0x49, 0x41], // E
    [0x7F, 0x09, 0x09, 0x09, 0x01], // F
    [0x3E, 0x41, 0x49, 0x49, 0x7A], // G
    [0x7F, 0x08, 0x08, 0x08, 0x7F], // H
    [0x00, 0x41, 0x7F, 0x41, 0x00], // I
    [0x20, 0x40, 0x41, 0x3F, 0x01], // J
    [0x7F, 0x08, 0x14, 0x22, 0x41], // K
    [0x7F, 0x40, 0x40, 0x40, 0x40], // L
    [0x7F, 0x02, 0x0C, 0x02, 0x7F], // M
    [0x7F, 0x04, 0x08, 0x10, 0x7F], // N
    [0x3E, 0x41, 0x41, 0x41, 0x3E], // O
    [0x7F, 0x09, 0x09, 0x09, 0x06], // P
    [0x3E, 0x41, 0x51, 0x21, 0x5E], // Q
    [0x7F, 0x09, 0x19, 0x29, 0x46], // R
    [0x46, 0x49, 0x49, 0x49, 0x31], // S
    [0x01, 0x01, 0x7F, 0x01, 0x01], // T
    [0x3F, 0x40, 0x40, 0x40, 0x3F], // U
    [0x0F, 0x30, 0x40, 0x30, 0x0F], // V
    [0x7F, 0x20, 0x18, 0x20, 0x7F], // W
    [0x63, 0x14, 0x08, 0x14, 0x63], // X
    [0x03, 0x04, 0x78, 0x04, 0x03], // Y
    [0x61, 0x51, 0x49, 0x45, 0x43], // Z
];

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

    /// Initialise the display with default value
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
        self.send_command(0xA4)?;

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

    fn fill_screen_with_value(&mut self, value: u8) -> Result<(), E> {
        // send proper command to reset screen

        // set the column addr range from 0 to 127
        self.send_command(0x20)?;
        self.send_command(0x01)?;

        // Column: addr, start, end
        self.send_command(0x21)?;
        self.send_command(0x00)?;
        self.send_command(0x7F)?; // 127

        // Page: addr, start, end
        self.send_command(0x22)?;
        self.send_command(0x00)?;
        self.send_command(0x07)?;

        // update screen value
        for _ in 0..8 {
            for _ in 0..128 {
                self.send_data(value)?;
            }
        }
        Ok(())
    }

    pub fn fill(&mut self) -> Result<(), E> {
        self.fill_screen_with_value(0x00)?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), E> {
        self.fill_screen_with_value(0xFF)?;
        Ok(())
    }

    fn set_cursor(&mut self, col: u8, page: u8, len: u8) -> Result<(), E> {
        // set the column addr range from 0 to 127
        self.send_command(0x20)?;
        self.send_command(0x01)?;

        // set column
        self.send_command(0x21)?;
        self.send_command(col)?;
        self.send_command(col + len)?;

        // set page
        self.send_command(0x22)?;
        self.send_command(page)?;
        self.send_command(page)?;

        Ok(())
    }

    fn write_char(&mut self, c: char) -> Result<(), E> {
        let index = match c {
            'A'..'Z' => (c as u8 - b'A' + 1) as usize,
            ' ' => 0,
            _ => return Ok(()),
        };

        for &byte in &FONT_5X8[index] {
            self.send_data(byte)?;
        }
        //self.send_data(0x00)?;

        Ok(())
    }

    pub fn draw_text(&mut self, text: &str, col: u8, page: u8) -> Result<(), E> {
        self.set_cursor(col, page, (text.len() * 5) as u8)?;

        for c in text.chars() {
            self.write_char(c)?;
            // self.set_cursor(col + 5, page)?;
        }
        Ok(())
    }
}
