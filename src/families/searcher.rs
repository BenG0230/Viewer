pub mod algorithms;
mod searcher_playback;
pub mod tracer;

use crate::playback::Playback;
use rand::seq::SliceRandom;
use searcher_playback::SearcherPlayback;

const ARRAY_LENGTH: u32 = 150;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Op {
    Read(usize),
    Compare(usize, usize),
    Swap(usize, usize),
    Finish(usize),
}

pub fn build(
    searcher_function: fn(&mut tracer::Tracer, u32) -> Option<usize>,
    random: bool,
) -> Box<dyn Playback> {
    let mut arr = (1..=ARRAY_LENGTH).collect::<Vec<u32>>();
    if random {
        let mut rng = rand::rng();
        arr.shuffle(&mut rng);
    }
    let value_to_find = rand::random_range(1..=ARRAY_LENGTH);

    let mut tracer = tracer::Tracer::new(arr.clone());
    match searcher_function(&mut tracer, value_to_find) {
        Some(val) => {
            assert_eq!(
                arr[val], value_to_find,
                "Value does not match searched value! {} - {}",
                arr[val], value_to_find
            );

            let mut operations = tracer.finish();
            operations.push(Op::Finish(val));

            Box::new(SearcherPlayback::new(value_to_find, arr, operations))
        }
        None => panic!("Value not found! {}", value_to_find),
    }
}
