use crate::families::sorter::tracer;

pub fn bubble_sort(t: &mut tracer::Tracer) {
    for i in 0..t.len() {
        let mut swapped = false;
        for j in 0..t.len() - 1 - i {
            if t.less_than(j + 1, j) {
                t.swap(j + 1, j);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
