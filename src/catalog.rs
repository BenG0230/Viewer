use crate::families::sorter;
use crate::playback::Playback;

pub struct CatalogEntry {
    name: &'static str,
    build_fn: fn() -> Box<dyn Playback>,
}

impl CatalogEntry {
    pub fn build(&self) -> Box<dyn Playback> {
        (self.build_fn)()
    }
}

pub static ENTRIES: &[CatalogEntry] = &[
    CatalogEntry {
        name: "Bubble Sort",
        build_fn: || sorter::build(sorter::algorithms::bubble_sort),
    },
    CatalogEntry {
        name: "Insertion Sort",
        build_fn: || sorter::build(sorter::algorithms::insertion_sort),
    },
    CatalogEntry {
        name: "Heap Sort",
        build_fn: || sorter::build(sorter::algorithms::heap_sort),
    },
    CatalogEntry {
        name: "Merge Sort",
        build_fn: || sorter::build(sorter::algorithms::merge_sort_start),
    },
    CatalogEntry {
        name: "Quick Sort",
        build_fn: || sorter::build(sorter::algorithms::quick_sort_start),
    },
];
