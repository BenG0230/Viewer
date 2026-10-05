#![allow(dead_code)]

pub mod colour;
mod shapes;

pub struct Frame<'a> {
    width: u32,
    height: u32,
    buffer: &'a mut [u8],
}

impl<'a> Frame<'a> {
    pub fn new(width: u32, height: u32, buffer: &'a mut [u8]) -> Self {
        // Buffer should have r,g,b,a for each pixel within frame
        assert_eq!((width * height * 4) as usize, buffer.len());

        Self {
            width,
            height,
            buffer,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn clear(&mut self, colour: [u8; 4]) {
        for px in self.buffer.chunks_exact_mut(4) {
            px.copy_from_slice(&colour);
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, colour: [u8; 4]) {
        if x >= self.width || y >= self.height {
            return;
        }

        let i = ((x + y * self.width) * 4) as usize;
        self.buffer[i..i + 4].copy_from_slice(&colour);
    }
}
