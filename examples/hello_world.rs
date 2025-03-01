//! Hello World!
//!

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::main;
use esp_hal::timer::timg::TimerGroup;
use log::info;

extern crate alloc;

use nfp1315::SSD1306;

#[main]
fn main() -> ! {
    // project generated with esp-generate version 0.2.2

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();

    esp_alloc::heap_allocator!(72 * 1024);

    // heap allocater
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    // Create an I2C instance
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .unwrap()
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22);

    let delay = Delay::new();

    // Create the driver structer
    let mut display = SSD1306::new(i2c, 0x3C);

    // Initialise the display, check for errors
    match display.init_display() {
        Ok(_) => {}
        Err(e) => info!("I2C e: {}", e),
    }

    loop {
        // Clear the screen
        display.clear();
        delay.delay_millis(500);

        // Draw hello World at 0, 0 (top-left)
        display.draw_text("HELLO WORLD", 0, 0);

        // Text wrap around the screen when it meet an edge
        display.draw_text("WARPED 0123456789", 80, 3);

        // Fill the screen
        display.fill();
        delay.delay_millis(500);
    }
}
