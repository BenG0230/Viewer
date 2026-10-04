use super::Op;
use crate::playback::Playback;

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
            Op::Swap(a, b) => self.data.swap(a, b),
            _ => {}
        }
    }

    pub fn reverse_op(&mut self, operation: Op) {
        match operation {
            Op::Swap(a, b) => self.data.swap(a, b),
            _ => {}
        }
    }
}

impl Playback for SorterPlayback {
    fn forward(&mut self) {
        // read operation under cursor and perform it
        let curr_op = self.trace[self.cursor];
        self.perform_op(curr_op);

        // increment cursor
        self.cursor += 1;
    }

    fn back(&mut self) {
        // decrement cursor
        self.cursor -= 1;

        // reverse operation under cursor
        let last_op = self.trace[self.cursor];
        self.reverse_op(last_op);
    }

    fn draw(&self, frame: &mut crate::gfx::Frame) {
        todo!()
    }
}
