#![no_std]
#![no_main]

mod display;
mod lcd_i2c;

use arduino_hal::prelude::*;
use display::{DisplayManager, HardwareDisplay};
use lcd_i2c::LcdI2c;
use panic_halt as _;

struct SerialDisplay<'a, W> {
    serial: &'a mut W,
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut led = pins.d13.into_output();

    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        100000,
    );

    arduino_hal::delay_ms(500);

    ufmt::uwriteln!(&mut serial, "Initializing LCD...").unwrap();
    let mut lcd = LcdI2c::new(i2c, 0x27).unwrap();
    let mut ui = DisplayManager::new(16, 2);

    let lines = [
        "Coil Winder v0",
        "Status: Ready",
        "This line is way too long for a 16 column screen!",
    ];

    ui.set_lines(&lines);

    let mut loop_counter = 0;

    loop {
        led.toggle();

        if loop_counter > 0 && loop_counter % 12 == 0 {
            let next_top = (ui.get_top() + 1) % lines.len();
            ui.set_top(next_top);
            ufmt::uwriteln!(&mut serial, "Scrolled to line {}", next_top).unwrap();
        }

        let _ = ui.draw(&mut lcd);
        ui.tick();
        loop_counter += 1;

        arduino_hal::delay_ms(250);
    }
}
