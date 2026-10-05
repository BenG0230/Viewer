use crate::families::sorter::tracer;

pub fn heap_sort(t: &mut tracer::Tracer) {
    for i in (0..=t.len() / 2).rev() {
        reheap(t.len(), i, t);
    }

    for curr_size in (1..t.len()).rev() {
        t.swap(0, curr_size);

        reheap(curr_size, 0, t);
    }
}

fn reheap(size: usize, root: usize, t: &mut tracer::Tracer) {
    let mut i = root;

    let l = 2 * i + 1;
    let r = 2 * i + 2;

    if l < size && t.less_than(i, l) {
        i = l;
    }

    if r < size && t.less_than(i, r) {
        i = r;
    }

    if i != root {
        t.swap(root, i);

        reheap(size, i, t);
    }
}
