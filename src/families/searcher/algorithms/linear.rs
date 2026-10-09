use crate::families::searcher::tracer;

pub fn linear_search(t: &mut tracer::Tracer, x: u32) -> Option<usize> {
    for i in 0..t.len() {
        if t.read(i) == x {
            return Some(i);
        }
    }

    return None;
}
