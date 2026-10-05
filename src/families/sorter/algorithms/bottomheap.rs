use crate::families::sorter::tracer;

pub fn heap_sort(t: &mut tracer::Tracer) {
    for i in (0..=t.len() / 2).rev() {
        reheap(t.len(), i, t);
    }

    for curr_size in (2..=t.len()).rev() {
        t.swap(0, curr_size - 1);

        reheap(curr_size - 1, 0, t);
    }
}

fn reheap(size: usize, root: usize, t: &mut tracer::Tracer) {
    let mut i = root;
    while 2 * i + 1 < size {
        if (2 * i + 1 < size) && (t.less_than(2 * i + 1, 2 * i + 2)) {
            i = 2 * i + 2;
        } else {
            i = 2 * i + 1;
        }
    }
}
