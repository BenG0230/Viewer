use crate::families::sorter::tracer;

pub fn merge_sort_start(t: &mut tracer::Tracer) {
    let mut temp: Vec<u32> = vec![0; t.len()];

    merge_sort(t, t.len(), &mut temp, 0);
}

fn merge_sort(t: &mut tracer::Tracer, size: usize, temp: &mut Vec<u32>, start: usize) {
    if size < 2 {
        return;
    }

    merge_sort(t, size / 2, temp, start);
    merge_sort(t, size - (size / 2), temp, start + size / 2);

    let mut min1 = start;
    let mut min2 = start + (size / 2);
    for i in 0..size {
        if (min2 >= start + size) || (min1 < start + (size / 2)) && !t.less_than(min2, min1) {
            temp[i] = t.read(min1);
            min1 += 1;
        } else {
            temp[i] = t.read(min2);
            min2 += 1;
        }
    }

    for i in 0..size {
        t.write(start + i, temp[i]);
    }
}
