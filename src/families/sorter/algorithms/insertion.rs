use crate::families::sorter::tracer;

pub fn insertion_sort(t: &mut tracer::Tracer) {
    for i in 1..t.len() {
        let mut j = i;

        while j > 0 && t.less_than(j, j - 1) {
            t.swap(j, j - 1);
            j -= 1;
        }
    }
}
