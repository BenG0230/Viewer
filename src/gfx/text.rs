use font8x8::legacy::BASIC_LEGACY;

use super::Frame;

impl<'a> Frame<'a> {
    pub fn draw_char(&mut self, x: u32, y: u32, ch: char, size: u32, colour: [u8; 4]) {
        if x >= self.width || y >= self.height || ch as usize >= BASIC_LEGACY.len() || size == 0 {
            return;
        }

        for row in 0..8 {
            let row_bitmap = &BASIC_LEGACY[ch as usize][row];
            for bit in 0..8 {
                if *row_bitmap & (1 << bit) != 0 {
                    self.fill_rect(x + bit * size, y + (row as u32) * size, size, size, colour);
                }
            }
        }
    }

    pub fn draw_text(
        &mut self,
        mut x: u32,
        y: u32,
        text: &'static str,
        size: u32,
        colour: [u8; 4],
    ) {
        if x >= self.width || y >= self.height || size == 0 {
            return;
        }

        for ch in text.chars() {
            self.draw_char(x, y, ch, size, colour);
            x += size * 8;
        }
    }
}
