use super::indexers::{Array, ConcreteSlice, LabelIndexer, PositionalIndexer};
use moc::elemset::range::MocRanges;
use moc::moc::range::RangeMOC;
use moc::qty::Hpx;
use std::ops::Range;

pub(crate) trait Indexing {
    fn sel(&self, indexer: LabelIndexer) -> Self;
    fn isel(&self, indexer: PositionalIndexer) -> Self;
}

pub(crate) trait SubsetMoc {
    fn slice(&self, slice: ConcreteSlice<isize>) -> Self;
    fn index(&self, array: Array<isize>) -> Self;
}

impl SubsetMoc for RangeMOC<u64, Hpx<u64>> {
    fn slice(&self, slice: ConcreteSlice<isize>) -> Self {
        if slice.step != 1 {
            panic!("Only step size 1 is supported, got {}", slice.step);
        }

        let mut start = slice.start;
        let mut stop = slice.stop;

        let delta_depth = 29 - self.depth_max();
        let shift = delta_depth << 1;

        let ranges = MocRanges::new_from(
            self.moc_ranges()
                .iter()
                .filter_map(|range: &Range<u64>| {
                    let range_size = ((range.end - range.start) >> shift) as isize;

                    if start >= range_size {
                        // range entirely before slice
                        start -= range_size;
                        stop -= range_size;

                        None
                    } else if stop > 0 {
                        // some overlap
                        let new_start = range.start + ((start as u64) << shift);
                        let new_end = if stop >= range_size {
                            range.end
                        } else {
                            range.start + ((stop as u64) << shift)
                        };

                        let new_range = Range {
                            start: new_start,
                            end: new_end,
                        };

                        if start >= range_size {
                            start -= range_size;
                        } else {
                            start = 0;
                        }

                        if stop >= range_size {
                            stop -= range_size;
                        } else {
                            stop = 0;
                        }

                        Some(new_range)
                    } else {
                        // slice exhausted
                        None
                    }
                })
                .collect::<Vec<Range<u64>>>(),
        );

        RangeMOC::new(self.depth_max(), ranges)
    }

    fn index(&self, array: Array<isize>) -> Self {
        let size = self.n_depth_max_cells() as usize;
        let normalized = array.normalize(size);
        let delta_depth = 29 - self.depth_max();
        let shift = delta_depth << 1;

        let slice_offsets = self
            .moc_ranges()
            .iter()
            .map(|range| ((range.end - range.start) >> shift) as usize)
            .scan(0, |acc, x| {
                let cur = *acc;
                *acc += x;
                Some((cur, *acc))
            })
            .collect::<Vec<(usize, usize)>>();
        let slice_starts = self
            .moc_ranges()
            .iter()
            .map(|range| range.start)
            .collect::<Vec<u64>>();

        let cell_ids: Vec<u64> = normalized
            .data
            .iter()
            .map(|&index| {
                let position = index as usize;
                if index >= size {
                    panic!("{index} is out of bounds");
                } else {
                    let slice_index = slice_offsets
                        .iter()
                        .position(|x| position >= x.0 && position < x.1)
                        .unwrap_or(slice_offsets.len() - 1);
                    let slice_start = slice_starts[slice_index] >> shift;
                    let selected =
                        slice_start + (index as u64 - (slice_offsets[slice_index].0 as u64));

                    selected
                }
            })
            .collect::<Vec<u64>>();

        RangeMOC::from_fixed_depth_cells(self.depth_max(), cell_ids.into_iter(), None)
    }
}
