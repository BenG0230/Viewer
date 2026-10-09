use super::Op;

pub struct Tracer {
    data: Vec<u32>,
    operations: Vec<Op>,
}

impl Tracer {
    pub fn new(data: Vec<u32>) -> Self {
        Self {
            data,
            operations: vec![],
        }
    }

    pub fn finish(self) -> Vec<Op> {
        self.operations
    }

    pub fn less_than(&mut self, a: usize, b: usize) -> bool {
        self.operations.push(Op::Compare(a, b));
        self.data[a] < self.data[b]
    }

    pub fn read(&mut self, a: usize) -> u32 {
        self.operations.push(Op::Read(a));
        self.data[a]
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.operations.push(Op::Swap(a, b));
        self.data.swap(a, b);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
