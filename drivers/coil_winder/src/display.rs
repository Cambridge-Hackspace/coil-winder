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
    front_buffer: [u8; 80],
    back_buffer: [u8; 80],
    needs_redraw: bool,
}

impl DisplayManager {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            top: 0,
            row_hashes: [0; 4],
            row_ticks: [0; 4],
            front_buffer: [0; 80],
            back_buffer: [b' '; 80],
            needs_redraw: true,
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

                for col in 0..(self.width as usize) {
                    let c = if len > (self.width as usize) {
                        let virt_len = len + 4;
                        let char_idx = (tick + col) % virt_len;
                        if char_idx < len {
                            line[char_idx]
                        } else {
                            b' '
                        }
                    } else {
                        if col < len {
                            line[col]
                        } else {
                            b' '
                        }
                    };

                    let fb_idx = r * (self.width as usize) + col;
                    self.back_buffer[fb_idx] = c;
                }
            } else {
                for col in 0..(self.width as usize) {
                    let fb_idx = r * (self.width as usize) + col;
                    self.back_buffer[fb_idx] = b' ';
                }
            }
        }

        if self.needs_redraw {
            self.front_buffer.fill(0);
        }

        for row in 0..self.height {
            let mut cursor_col = 255;
            for col in 0..(self.width as usize) {
                let fb_idx = (row as usize) * (self.width as usize) + col;
                let desired = self.back_buffer[fb_idx];

                if self.front_buffer[fb_idx] != desired {
                    if cursor_col != col {
                        display.set_cursor(row, col as u8)?;
                        cursor_col = col;
                    }
                    display.write_char(desired as char)?;
                    self.front_buffer[fb_idx] = desired;
                    cursor_col += 1;
                }
            }
        }

        self.needs_redraw = false;
        Ok(())
    }
}
