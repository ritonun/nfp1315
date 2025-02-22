# NFP1315-61A

Small library for the NFP1315-61A display. It use an SSD1306 driver, with an I2C communication bus. It aims at being very simple and minimalistic, and is compatible with [esp-hal](https://github.com/esp-rs/esp-hal), [embedded-hal](https://github.com/rust-embedded/embedded-hal), ...

## Usage
```rust
use nfp1315::SSD1306;

let mut display = SSD1306::new(i2c, 0x3C);
loop {
    display.clear();
}
```

## To Do
- [ ] Wrap text around edges
- [ ] Easy way to add custom icons
