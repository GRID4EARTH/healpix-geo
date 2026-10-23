use cdshealpix as healpix;

pub fn to_nested(hash: &u64) -> (u64, u8) {
    let (depth, hash_nested) = healpix::nested::from_zuniq(*hash);

    (hash_nested, depth)
}

pub fn to_ring(hash: &u64) -> (u64, u8) {
    let (hash_nested, depth) = to_nested(hash);

    let hash_ring = healpix::nested::get(depth).to_ring(hash_nested);

    (hash_ring, depth)
}

/// Re-encode a `zuniq` cell id as a Morton word.
///
/// Like [`to_nested`], `hash` must be a well-formed `zuniq` id.
pub fn to_morton(hash: &u64) -> u64 {
    let (hash_nested, depth) = to_nested(hash);

    mortie_core::from_nested(hash_nested, depth)
}
