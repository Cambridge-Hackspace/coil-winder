pub trait HardwareDisplay {
    type Error;
    fn set_cursor(&mut self, row: u8, col: u8) -> Result<(), Self::Error>;
    fn write_char(&mut self, c: char) -> Result<(), Self::Error>;
    fn clear(&mut self) -> Result<(), Self::Error>;
}

pub struct DisplayManager {
    width: u8,
    height: u8,
    top: usize,
    row_hashes: [u32; 4],
    row_ticks: [usize; 4],
}

impl DisplayManager {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            top: 0,
            row_hashes: [0; 4],
            row_ticks: [0; 4],
        }
    }

    pub fn set_top(&mut self, top: usize) {
        self.top = top;
    }

    pub fn get_top(&self) -> usize {
        self.top
    }

    pub fn tick(&mut self) {
        for tick in self.row_ticks.iter_mut() {
            *tick = tick.wrapping_add(1);
        }
    }

    pub fn draw<D: HardwareDisplay>(
        &mut self,
        display: &mut D,
        lines: &[&str],
    ) -> Result<(), D::Error> {
        let max_top = lines.len().saturating_sub(self.height as usize);
        self.top = self.top.min(max_top);

        for row in 0..self.height {
            display.set_cursor(row, 0)?;

            let line_idx = self.top + (row as usize);
            let r = row as usize;

            if line_idx < lines.len() {
                let line = lines[line_idx].as_bytes();
                let len = line.len();

                let mut hash: u32 = 5381;
                for &b in line {
                    hash = hash.wrapping_mul(33).wrapping_add(b as u32);
                }

                if hash != self.row_hashes[r] {
                    self.row_hashes[r] = hash;
                    self.row_ticks[r] = 0;
                }

                let tick = self.row_ticks[r];

                if len > (self.width as usize) {
                    let virt_len = len + 4;
                    for col in 0..(self.width as usize) {
                        let char_idx = (tick + col) % virt_len;
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
