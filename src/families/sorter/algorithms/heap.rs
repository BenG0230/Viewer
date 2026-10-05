use crate::families::sorter::tracer;

pub fn heap_sort(t: &mut tracer::Tracer) {
    for i in (0..t.len() / 2).rev() {
        reheap(t.len(), i, t);
    }

    for curr_size in (1..t.len()).rev() {
        t.swap(0, curr_size);

        reheap(curr_size, 0, t);
    }
}

fn reheap(size: usize, root: usize, t: &mut tracer::Tracer) {
    let mut i = root;

    // Follow largest path down tree
    while 2 * i + 1 < size {
        if 2 * i + 2 < size && t.less_than(2 * i + 1, 2 * i + 2) {
            i = 2 * i + 2;
        } else {
            i = 2 * i + 1;
        }
    }

    // Get to position where current root fits
    while i > root && !t.less_than(root, i) {
        i = (i - 1) / 2;
    }

    // Record path up to root
    let mut path = vec![i];
    while *path.last().unwrap() != root {
        path.push((path.last().unwrap() - 1) / 2);
    }
    path.reverse(); // make the path root -> i instead of i -> root 

    // swap down the path
    for pair in path.windows(2) {
        t.swap(pair[0], pair[1]);
    }
}
