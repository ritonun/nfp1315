//! This library provide a default configuration for the SSD1306, adapted to the NFP1315-61A display (128x64)
//! # Usage
//! ```
//! use nfp1315::SSD1306;
//! // the library is build around embedded-hal, so it is compatible with many µC
//! // you can create an I2C instance with your board and SSD1306 will take ownership of it
//! // SSD1306 is not multi-bus
//! let mut display = SSD1306::new(i2c, 0x3C); // create the SSD1306 struct with the NFP1315-61A address: 0x3C
//! display.init_display(); // initialise the display
//!
//! display.clear(); // clear the display (Black)
//! display.fill(); // fill the display (White)
//! display.draw_text(text: &str, col: u8, page: u8); // draw text to the display at the position (col, page)
//!
//! // All functions return a `Result<(), Error>
//! match display.draw_text("Hello World", 50, 0) {
//!     Ok(_) => {}
//!     Err(e) => //...
//! }
//! ```

#![no_std]

use embedded_hal::i2c::I2c;

const FONT_5X8: [[u8; 5]; 37] = [
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
    [0x3E, 0x51, 0x49, 0x45, 0x3E], // 0
    [0x00, 0x42, 0x7F, 0x40, 0x00], // 1
    [0x62, 0x51, 0x49, 0x49, 0x46], // 2
    [0x22, 0x41, 0x49, 0x49, 0x36], // 3
    [0x18, 0x14, 0x12, 0x7F, 0x10], // 4
    [0x27, 0x45, 0x45, 0x45, 0x39], // 5
    [0x3E, 0x49, 0x49, 0x49, 0x32], // 6
    [0x01, 0x71, 0x09, 0x05, 0x03], // 7
    [0x36, 0x49, 0x49, 0x49, 0x36], // 8
    [0x26, 0x49, 0x49, 0x49, 0x3E], // 9
];

/// Store a blocking I2C instance and the adress of the device
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

        // set the column addr range from 0 to 127
        self.send_command(0x20)?;
        self.send_command(0x01)?;

        // turn display on
        self.send_command(0xAF)?;

        // clear display
        self.clear()?;

        Ok(())
    }

    /// Write the same value to all pixel of the display
    fn fill_screen_with_value(&mut self, value: u8) -> Result<(), E> {
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

    /// Fill the screen (White)
    pub fn fill(&mut self) -> Result<(), E> {
        self.fill_screen_with_value(0xFF)?;
        Ok(())
    }

    /// Clear the screen (Black)
    pub fn clear(&mut self) -> Result<(), E> {
        self.fill_screen_with_value(0x00)?;
        Ok(())
    }

    /// Set the cursor for the start and end of the drawing zone
    fn set_cursor(&mut self, col: u8, page: u8, len: u8) -> Result<(), E> {
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

    /// Write a char on screen
    fn write_char(&mut self, c: char) -> Result<(), E> {
        let index = match c {
            'A'..'Z' => (c as u8 - b'A' + 1) as usize,
            'a'..'z' => (c.to_ascii_uppercase() as u8 - b'A' + 1) as usize,
            '0'..'9' => (c as u8 - b'0' + 27) as usize,
            ' ' => 0,
            _ => return Ok(()),
        };

        for &byte in &FONT_5X8[index] {
            self.send_data(byte)?;
        }
        //self.send_data(0x00)?; // to use if we want to espace the character by 1 pixel

        Ok(())
    }

    /// Draw text on the screen, text wrap around the edges
    pub fn draw_text(&mut self, text: &str, mut col: u8, mut page: u8) -> Result<(), E> {
        // self.set_cursor(col, page, (text.len() * 5) as u8)?;

        for c in text.chars() {
            if col + 5 > 128 {
                col = 0;
                page += 1;
            }
            self.set_cursor(col, page, 5)?;
            self.write_char(c)?;
            col += 5;
        }
        Ok(())
    }
}
