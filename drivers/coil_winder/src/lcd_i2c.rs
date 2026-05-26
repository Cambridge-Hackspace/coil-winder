use crate::display::HardwareDisplay;
use embedded_hal::i2c::I2c;

/// A driver for an HD44780 LCD connected via a PCF8574 I2C backpack.
pub struct LcdI2c<I2C> {
    i2c: I2C,
    address: u8,
    backlight: u8, // 0x08 for ON, 0x00 for OFF
}

impl<I2C: I2c> LcdI2c<I2C> {
    pub fn new(i2c: I2C, address: u8) -> Result<Self, I2C::Error> {
        let mut lcd = Self {
            i2c,
            address,
            backlight: 0x08,
        };
        lcd.init()?;
        Ok(lcd)
    }

    /// Sends one nibble + register select + backlight state.
    fn write_nibble(&mut self, data: u8, rs: u8) -> Result<(), I2C::Error> {
        let byte = (data & 0xF0) | rs | self.backlight;

        self.i2c.write(self.address, &[byte & !0x04])?;
        arduino_hal::delay_us(10);

        self.i2c.write(self.address, &[byte | 0x04])?;
        arduino_hal::delay_us(10);

        self.i2c.write(self.address, &[byte & !0x04])?;
        arduino_hal::delay_us(100);

        Ok(())
    }

    /// Sends a full byte as two consecutive nibbles.
    fn write_byte(&mut self, data: u8, rs: u8) -> Result<(), I2C::Error> {
        self.write_nibble(data & 0xF0, rs)?;
        self.write_nibble((data << 4) & 0xF0, rs)?;
        Ok(())
    }

    fn command(&mut self, cmd: u8) -> Result<(), I2C::Error> {
        self.write_byte(cmd, 0) // RS = 0 for commands
    }

    fn data(&mut self, data: u8) -> Result<(), I2C::Error> {
        self.write_byte(data, 1) // RS = 1 for data
    }

    /// HD44780 initialization sequence.
    fn init(&mut self) -> Result<(), I2C::Error> {
        arduino_hal::delay_ms(50);

        // force reset into 4-bit mode
        self.write_nibble(0x30, 0)?;
        arduino_hal::delay_ms(5);
        self.write_nibble(0x30, 0)?;
        arduino_hal::delay_us(150);
        self.write_nibble(0x30, 0)?;

        self.write_nibble(0x20, 0)?; // set to 4-bit mode

        self.command(0x28)?; // function set: 4-bit, 2 lines, 5x8 font
        self.command(0x0c)?; // display ON, cursor OF, blink OFF
        self.command(0x06)?; // entry mode: increment, no shift

        self.clear_display()?;

        Ok(())
    }

    pub fn clear_display(&mut self) -> Result<(), I2C::Error> {
        self.command(0x01)?;
        arduino_hal::delay_ms(2);
        Ok(())
    }
}

impl<I2C: I2c> HardwareDisplay for LcdI2c<I2C> {
    type Error = I2C::Error;

    fn set_cursor(&mut self, row: u8, col: u8) -> Result<(), Self::Error> {
        // standard RAM offsets for HD44780 displays
        let row_offsets = [0x00, 0x40, 0x14, 0x54];
        let clamped_row = (row as usize).min(3);
        self.command(0x80 | (col + row_offsets[clamped_row]))
    }

    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.data(c as u8)
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        self.clear_display()
    }
}
