pub mod algorithms;
mod sorter_playback;
pub mod tracer;

use crate::playback::Playback;
use rand::seq::SliceRandom;
use sorter_playback::SorterPlayback;

const ARRAY_LENGTH: u32 = 100;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Op {
    Read(usize),
    Compare(usize, usize),
    Swap(usize, usize),
}

// Make the tracer and run a method over it and create a player around the returned operation list
pub fn build(sorter_function: fn(&mut tracer::Tracer)) -> Box<dyn Playback> {
    let mut rng = rand::rng();
    let mut arr = (1..=ARRAY_LENGTH).collect::<Vec<u32>>();
    arr.shuffle(&mut rng);

    // Create a tracer and run the algorithms implementation over it
    let mut tracer = tracer::Tracer::new(arr.clone());
    sorter_function(&mut tracer);

    // Consume the tracer and create a playback off the operation list and initial arr
    Box::new(SorterPlayback::new(arr, tracer.finish()))
}
