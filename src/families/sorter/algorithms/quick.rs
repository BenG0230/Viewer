use crate::families::sorter::tracer;

pub fn quick_sort_start(t: &mut tracer::Tracer) {
    quick_sort(t, t.len(), 0);
}

fn quick_sort(t: &mut tracer::Tracer, size: usize, start: usize) {
    if size < 2 {
        return;
    }

    // Randomised pivot
    let mut pivot = start + rand::random_range(0..size);
    pivot = partition(t, start, start + size - 1, pivot);

    quick_sort(t, pivot - start, start);
    quick_sort(t, start + size - pivot - 1, pivot + 1);
}

fn partition(t: &mut tracer::Tracer, mut left: usize, mut right: usize, mut pivot: usize) -> usize {
    while left < right {
        while t.less_than(left, pivot) {
            left += 1;
        }
        while t.less_than(pivot, right) {
            right -= 1;
        }

        t.swap(left, right);

        if left == pivot {
            pivot = right;
        } else if right == pivot {
            pivot = left;
        }

        if left < pivot {
            left += 1;
        }

        if right > pivot {
            right -= 1;
        }
    }

    pivot
}
