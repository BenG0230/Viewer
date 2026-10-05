pub mod algorithms;
mod sorter_playback;
pub mod tracer;

use crate::playback::Playback;
use rand::seq::SliceRandom;
use sorter_playback::SorterPlayback;

const ARRAY_LENGTH: u32 = 150;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Op {
    Read(usize),
    Write(usize, u32, u32), // index, new val, old val
    Compare(usize, usize),
    Swap(usize, usize),
    Finish,
}

// Make the tracer and run a method over it and create a player around the returned operation list
pub fn build(sorter_function: fn(&mut tracer::Tracer)) -> Box<dyn Playback> {
    let mut rng = rand::rng();
    let arr_orig = (1..=ARRAY_LENGTH).collect::<Vec<u32>>();
    let mut arr = arr_orig.clone();
    arr.shuffle(&mut rng);

    // Create a tracer and run the algorithms implementation over it
    let mut tracer = tracer::Tracer::new(arr.clone());
    sorter_function(&mut tracer);

    // Consume the tracer and create a playback off the operation list and initial arr
    let mut operations = tracer.finish();
    operations.push(Op::Finish);

    Box::new(SorterPlayback::new(arr, operations))
}
