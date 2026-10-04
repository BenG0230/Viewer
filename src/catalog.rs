use crate::families::sorter;
use crate::families::sorter::algorithms::bubble::bubble_sort;
use crate::playback::Playback;

struct CatalogEntry {
    name: &'static str,
    build_fn: fn() -> Box<dyn Playback>,
}

pub static ENTRIES: &[CatalogEntry] = &[CatalogEntry {
    name: "Bubble Sort",
    build_fn: || sorter::build(bubble_sort),
}];
