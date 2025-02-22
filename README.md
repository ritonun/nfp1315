# NFP1315-61A

Small library for the NFP1315-61A display. It use an SSD1306 driver, with an I2C communication bus. It aims at being very simple and minimalistic, and is compatible with [esp-hal](https://github.com/esp-rs/esp-hal), [embedded-hal](https://github.com/rust-embedded/embedded-hal), ...

## Display
This display is 128x64 pixels. To draw text, you need to specify the position by the column and page. There is 8 page, each 8 pixel tall, wich correspond to the size of one character.
```
0        32        64        96       127
  |---------|---------|---------|---------|
0 |                                         |
1 |                                         |
2 |                                         |
3 |                                         |
4 |                                         |
5 |                                         |
6 |                                         |
7 |                                         |
  |---------|---------|---------|---------|

```

## Usage
Create the SSD1306 struct:
```rust
use nfp1315::SSD1306; // import the library

// the library is build around embedded-hal, so it is compatible with many µC
// you can create an I2C instance with your board and SSD1306 will take ownership of it
// SSD1306 is not multi-bus
let mut display = SSD1306::new(i2c, 0x3C); // create the SSD1306 struct with the NFP1315-61A address: 0x3C
```

Functions:
```rust
display.clear(); // clear the display (Black)
display.fill(); // fill the display (White)
display.draw_text(text: &str, col: u8, page: u8); // draw text to the display at the position (col, page)
```

All function of SSD1306 return a `Result<(), Error>`. You can catch it with a match statement.
```rust
match display.draw_text("Hello World", 50, 0) {
    Ok(_) => {}
    Err(e) => //...
}
```

## To Do
- [ ] Easy way to add custom icons
