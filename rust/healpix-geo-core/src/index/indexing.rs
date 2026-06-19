use super::indexers::{Array, ConcreteSlice, LabelIndexer, PositionalIndexer};
use moc::elemset::range::MocRanges;
use moc::moc::range::RangeMOC;
use moc::qty::Hpx;
use std::ops::Range;

pub trait Indexing {
    fn sel(&self, indexer: &LabelIndexer) -> (Self, PositionalIndexer)
    where
        Self: Sized;

    fn isel(&self, indexer: &PositionalIndexer) -> Self
    where
        Self: Sized;
}

pub trait PositionIndexing {
    fn position_slice(&self, slice: &ConcreteSlice<isize>) -> Self;
    fn position_index(&self, array: &Array<isize>) -> Self;
}

pub trait LabelIndexing {
    fn label_slice(&self, slice: ConcreteSlice<u64>) -> (Self, ConcreteSlice<usize>)
    where
        Self: Sized;

    fn label_index(&self, array: &Array<u64>) -> (Self, Array<usize>)
    where
        Self: Sized;
}

impl PositionIndexing for RangeMOC<u64, Hpx<u64>> {
    fn position_slice(&self, slice: &ConcreteSlice<isize>) -> Self {
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

    fn position_index(&self, array: &Array<isize>) -> Self {
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
                let position = index;
                if index >= size {
                    panic!("{index} is out of bounds");
                } else {
                    let slice_index = slice_offsets
                        .iter()
                        .position(|x| position >= x.0 && position < x.1)
                        .unwrap_or(slice_offsets.len() - 1);
                    let slice_start = slice_starts[slice_index] >> shift;

                    slice_start + (index as u64 - (slice_offsets[slice_index].0 as u64))
                }
            })
            .collect::<Vec<u64>>();

        RangeMOC::from_fixed_depth_cells(self.depth_max(), cell_ids.into_iter(), None)
    }
}

pub(crate) fn range_offsets(moc: &RangeMOC<u64, Hpx<u64>>) -> Vec<usize> {
    let relative_depth = 29 - moc.depth_max();

    moc.moc_ranges()
        .iter()
        .map(|r| ((r.end - r.start) >> (relative_depth << 1)) as usize)
        .scan(0, |state, x| {
            let val = *state;
            *state += x;
            Some(val)
        })
        .collect()
}

impl LabelIndexing for RangeMOC<u64, Hpx<u64>> {
    fn label_slice(&self, slice: ConcreteSlice<u64>) -> (Self, ConcreteSlice<usize>) {
        let depth = self.depth_max();
        let offsets = range_offsets(self);

        let (slices, ranges): (Vec<_>, Vec<_>) = self
            .moc_ranges()
            .iter()
            .enumerate()
            .filter_map(|(index, range)| {
                let offset = offsets[index];

                let relative_depth = 29 - depth;
                let range_start = range.start >> (relative_depth << 1);
                let range_end = range.end >> (relative_depth << 1);

                if (slice.start < range_end) && (slice.stop >= range_start) {
                    let pos_slice = ConcreteSlice {
                        start: slice.start.saturating_sub(range_start) as usize + offset,
                        stop: (slice.stop + 1).min(range_end).saturating_sub(range_start) as usize
                            + offset,
                        step: slice.step as usize,
                    };

                    let new_range = Range {
                        start: slice.start.max(range_start) << (relative_depth << 1),
                        end: (slice.stop + 1).min(range_end) << (relative_depth << 1),
                    };

                    Some((pos_slice, new_range))
                } else {
                    None
                }
            })
            .unzip();

        let joined_slice = ConcreteSlice::join(slices);

        let new_moc: RangeMOC<u64, Hpx<u64>> = RangeMOC::new(depth, MocRanges::new_from(ranges));

        (new_moc, joined_slice)
    }

    fn label_index(&self, array: &Array<u64>) -> (Self, Array<usize>) {
        let depth = self.depth_max();
        let offsets = range_offsets(self);

        let delta_depth = 29 - depth;
        let shift = delta_depth << 1;

        let ranges = self
            .moc_ranges()
            .iter()
            .map(|x| Range {
                start: x.start >> shift,
                end: x.end >> shift,
            })
            .collect::<Vec<_>>();

        let (positions, cell_ids): (Vec<_>, Vec<_>) = array
            .data
            .iter()
            .map(|&hash| {
                let range_index = ranges
                    .iter()
                    .position(|r: &Range<u64>| r.contains(&hash))
                    .expect("Cannot find {hash}");

                let position = (hash - ranges[range_index].start) as usize + offsets[range_index];

                (position, hash)
            })
            .unzip();

        let new_moc = RangeMOC::from_fixed_depth_cells(depth, cell_ids.into_iter(), None);

        (new_moc, Array { data: positions })
    }
}
