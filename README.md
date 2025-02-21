# NFP1315-61A

Small library for the NFP1315-61A display. It use an SSD1306 driver, with an I2C communication bus.

## Usage
```rust
use nfp1315::SSD1306;

let mut display = SSD1306::new(i2c, 0x3C);
loop {
    display.clear();
}
```
