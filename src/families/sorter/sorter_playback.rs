use super::Op;
use crate::{gfx::colour, playback::Playback};

const MARGIN: u32 = 25;

pub struct SorterPlayback {
    input: Vec<u32>,
    data: Vec<u32>,
    trace: Vec<Op>,
    cursor: usize,
}

impl SorterPlayback {
    pub fn new(input: Vec<u32>, trace: Vec<Op>) -> Self {
        Self {
            data: input.clone(),
            input,
            trace,
            cursor: 0,
        }
    }

    pub fn perform_op(&mut self, operation: Op) {
        match operation {
            Op::Write(a, b, _) => self.data[a] = b,
            Op::Swap(a, b) => self.data.swap(a, b),
            _ => {}
        }
    }

    pub fn reverse_op(&mut self, operation: Op) {
        match operation {
            Op::Write(a, _, b) => self.data[a] = b,
            Op::Swap(a, b) => self.data.swap(a, b),
            _ => {}
        }
    }

    pub fn get_bar_col(&self, i: usize) -> [u8; 4] {
        match self.trace[self.cursor] {
            Op::Read(a) => {
                if i == a {
                    return colour::YELLOW;
                }
            }
            Op::Write(a, ..) => {
                if i == a {
                    return colour::PURPLE;
                }
            }
            Op::Compare(a, b) => {
                if i == a || i == b {
                    return colour::BLUE;
                }
            }
            Op::Swap(a, b) => {
                if i == a || i == b {
                    return colour::RED;
                }
            }
            Op::Finish => {
                return colour::GREEN;
            }
            _ => {}
        }

        return colour::WHITE;
    }
}

impl Playback for SorterPlayback {
    fn forward(&mut self) {
        // read operation under cursor and perform it
        let curr_op = self.trace[self.cursor];
        self.perform_op(curr_op);

        // increment cursor
        if self.cursor != self.trace.len() - 1 {
            self.cursor += 1;
        }
    }

    fn back(&mut self) {
        // decrement cursor
        if self.cursor != 0 {
            self.cursor -= 1;

            // reverse operation under cursor
            let last_op = self.trace[self.cursor];
            self.reverse_op(last_op);
        }
    }

    fn draw(&self, frame: &mut crate::gfx::Frame) {
        let useable_w = frame.get_width() - 2 * MARGIN;
        let useable_h = frame.get_height() - 2 * MARGIN;
        let data_len = self.data.len();
        let bar_w = useable_w as f32 / data_len as f32;

        for i in 0..data_len {
            let bar_val = self.data[i];
            let bar_col = self.get_bar_col(i);

            let bar_h = (useable_h as f32 * (bar_val as f32 / data_len as f32)) as u32;

            let x = MARGIN + (bar_w * i as f32) as u32;
            let y = MARGIN + (useable_h - bar_h);

            frame.fill_rect(x, y, bar_w as u32, bar_h, bar_col);
        }
    }
}
