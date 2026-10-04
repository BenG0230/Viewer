use std::cmp::min;

use super::Frame;

impl<'a> Frame<'a> {
    pub fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, colour: [u8; 4]) {
        if x >= self.width || y >= self.height || w == 0 || h == 0 {
            return;
        }

        let right = min(x + w, self.width);
        let bottom = min(y + h, self.height);
        let byte_length = ((right - x) * 4) as usize;

        for y_i in y..bottom {
            let i = ((x + y_i * self.width) * 4) as usize;

            for px in self.buffer[i..i + byte_length].chunks_exact_mut(4) {
                px.copy_from_slice(&colour);
            }
        }
    }
}
