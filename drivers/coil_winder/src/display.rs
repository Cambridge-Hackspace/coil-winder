pub trait HardwareDisplay {
    type Error;
    fn set_cursor(&mut self, row: u8, col: u8) -> Result<(), Self::Error>;
    fn write_char(&mut self, c: char) -> Result<(), Self::Error>;
    fn clear(&mut self) -> Result<(), Self::Error>;
}

pub struct DisplayManager<'a> {
    width: u8,
    height: u8,
    lines: &'a [&'a str],
    top: usize,
    marquee_tick: usize,
}

impl<'a> DisplayManager<'a> {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            lines: &[],
            top: 0,
            marquee_tick: 0,
        }
    }

    pub fn set_lines(&mut self, lines: &'a [&'a str]) {
        self.lines = lines;
        self.top = 0;
        self.marquee_tick = 0;
    }

    pub fn set_top(&mut self, top: usize) {
        let max_top = if self.lines.is_empty() {
            0
        } else {
            self.lines.len().saturating_sub(1)
        };
        self.top = top.min(max_top);
    }

    pub fn get_top(&self) -> usize {
        self.top
    }

    pub fn tick(&mut self) {
        self.marquee_tick = self.marquee_tick.wrapping_add(1);
    }

    pub fn draw<D: HardwareDisplay>(&self, display: &mut D) -> Result<(), D::Error> {
        for row in 0..self.height {
            display.set_cursor(row, 0)?;

            let line_idx = self.top + (row as usize);

            if line_idx < self.lines.len() {
                let line = self.lines[line_idx].as_bytes();
                let len = line.len();

                if len > (self.width as usize) {
                    let virt_len = len + 4;
                    for col in 0..(self.width as usize) {
                        let char_idx = (self.marquee_tick + col) % virt_len;
                        let c = if char_idx < len {
                            line[char_idx] as char
                        } else {
                            ' '
                        };
                        display.write_char(c)?;
                    }
                } else {
                    for col in 0..(self.width as usize) {
                        let c = if col < len { line[col] as char } else { ' ' };
                        display.write_char(c)?;
                    }
                }
            } else {
                for _ in 0..self.width {
                    display.write_char(' ')?;
                }
            }
        }
        Ok(())
    }
}
