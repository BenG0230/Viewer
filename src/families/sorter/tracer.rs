use super::Op;

// used by the algorithm implementation to capture all operations on the data
pub(super) struct Tracer {
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

    // Called after algorithm is run, moves operation out and consumes the tracer
    pub fn finish(self) -> Vec<Op> {
        self.operations
    }

    // Implementation methods - used by the implementation so all operations on the data are caught

    pub fn read(&mut self, a: usize) -> u32 {
        self.operations.push(Op::Read(a));
        self.data[a]
    }

    pub fn less_than(&mut self, a: usize, b: usize) -> bool {
        self.operations.push(Op::Compare(a, b));
        self.data[a] < self.data[b]
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.operations.push(Op::Swap(a, b));
        self.data.swap(a, b);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
