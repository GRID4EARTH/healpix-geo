use cdshealpix as healpix;

pub fn to_zuniq(hash: &u64, depth: &u8) -> u64 {
    let hash_nested = healpix::nested::get(*depth).from_ring(*hash);

    healpix::nested::to_zuniq_unsafe(*depth, hash_nested)
}

pub fn to_nested(hash: &u64, depth: &u8) -> u64 {
    healpix::nested::get(*depth).from_ring(*hash)
}

/// Pack a ring cell index at `depth` into a Morton word.
///
/// # Panics
/// Panics if `depth > 29` or if `hash` is out of range at `depth`.
pub fn to_morton(hash: &u64, depth: &u8) -> u64 {
    mortie_core::from_nested(to_nested(hash, depth), *depth)
}
