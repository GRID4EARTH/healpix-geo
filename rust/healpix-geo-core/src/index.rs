use crate::ellipsoid::{Ellipsoid, ReferenceBody};
use moc::{qty::Hpx, moc::range::RangeMOC};

#[derive(PartialEq, Debug, Clone)]
struct MOCIndex {
    moc: RangeMoc<u64, Hpx<u64>>,
    ellipsoid: Ellipsoid,
}
