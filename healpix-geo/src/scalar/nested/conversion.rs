use cdshealpix as healpix;

pub fn to_zuniq(hash: &u64, depth: &u8) -> u64 {
    healpix::nested::to_zuniq_unsafe(*depth, *hash)
}

pub fn to_ring(hash: &u64, depth: &u8) -> u64 {
    healpix::nested::get(*depth).to_ring(*hash)
}

/// Pack a nested cell index at `depth` into a Morton word.
///
/// # Panics
/// Panics if `depth > 29` or if `hash` is out of range at `depth`.
pub fn to_morton(hash: &u64, depth: &u8) -> u64 {
    mortie_core::from_nested(*hash, *depth)
}

/// Pack a depth-29 nested cell index into a max-encoded Morton *point* word.
///
/// A point word marks a raw coordinate cast to maximum resolution with no
/// area claim; it sorts after every area cell of the same body and unpacks
/// (via `morton::conversion::to_nested`) to its depth-29 nested cell like any
/// other word.
///
/// # Panics
/// Panics if `hash` is out of range at depth 29.
pub fn to_morton_point(hash: &u64) -> u64 {
    mortie_core::decimal_morton::from_nested_point(*hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_morton_golden() {
        // words derived from the packed-word layout documented in
        // mortie-core ([4-bit prefix][27×2-bit body][6-bit suffix])
        assert_eq!(to_morton(&0, &0), 1152921504606846976);
        assert_eq!(to_morton(&0, &2), 1152921504606846978);
        assert_eq!(to_morton(&164, &3), 4107282860161892355);
        assert_eq!(to_morton(&3071, &4), 14983475960261640196);
        assert_eq!(to_morton(&11, &0), 13835058055282163712);
    }

    #[test]
    fn test_to_morton_point_golden() {
        let nested = 164u64 << (2 * 26);

        assert_eq!(to_morton_point(&nested), 4107282860161892400);
        // the point word and the depth-29 area word of the same cell differ
        assert_ne!(to_morton_point(&nested), to_morton(&nested, &29));
    }
}
