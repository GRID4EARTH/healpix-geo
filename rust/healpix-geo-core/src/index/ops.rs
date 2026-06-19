use super::indexers::ConcreteSlice;
use super::indexing::range_offsets;
use moc::moc::range::RangeMOC;
use moc::qty::Hpx;

pub(crate) enum JoinOp {
    Intersection,
}

pub(crate) trait JoinOps {
    fn join(&self, other: &Self, method: JoinOp) -> (Vec<ConcreteSlice<isize>>, Self);
}

impl JoinOps for RangeMOC<u64, Hpx<u64>> {
    fn join(&self, other: &Self, method: JoinOp) -> (Vec<ConcreteSlice<isize>>, Self) {
        let depth = self.depth_max();
        let relative_depth = 29 - depth;
        let shift = relative_depth << 1;

        let offsets: Vec<usize> = range_offsets(self);

        let new_region = self.intersection(other);

        let slices = match method {
            JoinOp::Intersection => other
                .moc_ranges()
                .iter()
                .filter_map(|range_o| {
                    let start_o = range_o.start >> shift;
                    let end_o = range_o.end >> shift;

                    let slices: Vec<_> = self
                        .moc_ranges()
                        .iter()
                        .enumerate()
                        .filter_map(|(index, range_s)| {
                            let start_s = range_s.start >> shift;
                            let end_s = range_s.end >> shift;
                            let offset = offsets[index];

                            if (start_o <= end_s) && (end_o >= start_s) {
                                let pos_slice = ConcreteSlice {
                                    start: start_o.saturating_sub(start_s) as isize
                                        + offset as isize,
                                    stop: end_o.min(end_s).saturating_sub(start_s) as isize
                                        + offset as isize,
                                    step: 1,
                                };

                                Some(pos_slice)
                            } else {
                                None
                            }
                        })
                        .collect();

                    if !slices.is_empty() {
                        Some(ConcreteSlice::join(slices))
                    } else {
                        None
                    }
                })
                .collect::<Vec<ConcreteSlice<isize>>>(),
        };

        (slices, new_region)
    }
}
