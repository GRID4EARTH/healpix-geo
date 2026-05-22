use crate::ellipsoid::{Ellipsoid, ReferenceBody};
use moc::{moc::range::RangeMOC, qty::Hpx};

#[derive(Debug, Clone)]
struct MOCIndex {
    moc: RangeMOC<u64, Hpx<u64>>,
    ellipsoid: Ellipsoid,
}
