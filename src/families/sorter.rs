mod algorithms;
mod tracer;

#[derive(Clone, Copy, Debug)]
enum Op {
    Read(usize),
    Compare(usize, usize),
    Swap(usize, usize),
}
