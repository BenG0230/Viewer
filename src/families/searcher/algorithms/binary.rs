use crate::families::searcher::tracer;

pub fn binary_search(t: &mut tracer::Tracer, x: u32) -> Option<usize> {
    let mut low = 0;
    let mut high = t.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        let mid_val = t.read(mid);

        if mid_val == x {
            return Some(mid);
        }

        if mid_val < x {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    return None;
}
