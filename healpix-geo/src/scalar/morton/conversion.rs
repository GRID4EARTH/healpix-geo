use cdshealpix as healpix;

/// The deepest refinement level a Morton word (or a `zuniq` id) can encode.
const MAX_DEPTH: u8 = 29;

/// Pack a nested cell index at `depth` into a Morton word.
///
/// # Panics
/// Panics if `depth > 29` or if `hash` is out of range at `depth`.
pub fn from_nested(hash: &u64, depth: &u8) -> u64 {
    mortie_core::from_nested(*hash, *depth)
}

/// Pack a depth-29 nested cell index into a max-encoded *point* word.
///
/// A point word marks a raw coordinate cast to maximum resolution with no
/// area claim; it sorts after every area cell of the same body and unpacks
/// (via [`to_nested`]) to its depth-29 nested cell like any other word.
///
/// # Panics
/// Panics if `hash` is out of range at depth 29.
pub fn from_nested_point(hash: &u64) -> u64 {
    mortie_core::decimal_morton::from_nested_point(*hash)
}

/// Unpack a Morton word into its nested cell index and depth.
///
/// Returns `None` if the word does not decode (the empty sentinel `0`, or an
/// invalid base-cell prefix). A max-encoded *point* word unpacks to its
/// depth-29 nested cell just like an area cell.
pub fn to_nested(hash: &u64) -> Option<(u64, u8)> {
    mortie_core::to_nested(*hash).map(|(depth, nested)| (nested, depth))
}

/// Pack a ring cell index at `depth` into a Morton word.
///
/// # Panics
/// Panics if `depth > 29` or if `hash` is out of range at `depth`.
pub fn from_ring(hash: &u64, depth: &u8) -> u64 {
    let nested = healpix::nested::get(*depth).from_ring(*hash);

    mortie_core::from_nested(nested, *depth)
}

/// Unpack a Morton word into its ring cell index and depth.
///
/// Returns `None` if the word does not decode.
pub fn to_ring(hash: &u64) -> Option<(u64, u8)> {
    let (nested, depth) = to_nested(hash)?;

    Some((healpix::nested::get(depth).to_ring(nested), depth))
}

/// Re-encode a `zuniq` cell id as a Morton word.
///
/// Returns `None` if `hash` is not a valid `zuniq` cell id. `cdshealpix`'s
/// `from_zuniq` is infallible in the type system but panics for those — `0`
/// underflows the depth computation, and an id whose sentinel bit sits at an
/// odd position or whose cell index is out of range at the encoded depth
/// (`u64::MAX`, for instance) blows up further down — so the id is checked
/// here rather than passed straight through.
pub fn from_zuniq(hash: &u64) -> Option<u64> {
    // a well-formed zuniq id has its sentinel bit at 2·(29 − depth); note
    // that `0` has 64 trailing zeros and is rejected by the upper bound
    let trailing = hash.trailing_zeros();
    if !trailing.is_multiple_of(2) || trailing > 2 * u32::from(MAX_DEPTH) {
        return None;
    }

    let (depth, nested) = healpix::nested::from_zuniq(*hash);
    if nested >= 12u64 << (2 * depth) {
        return None;
    }

    Some(mortie_core::from_nested(nested, depth))
}

/// Re-encode a Morton word as a `zuniq` cell id.
///
/// Returns `None` if the word does not decode.
pub fn to_zuniq(hash: &u64) -> Option<u64> {
    let (nested, depth) = to_nested(hash)?;

    Some(healpix::nested::to_zuniq(depth, nested))
}

/// Whether `hash` is a max-encoded *point* word rather than an area cell.
///
/// Total over every bit pattern — the suffix region alone decides the kind —
/// so this says nothing about whether the word decodes or is canonical; pair
/// it with [`is_canonical`] for untrusted input.
pub fn is_point(hash: &u64) -> bool {
    use mortie_core::decimal_morton::{Kind, kind_of};

    kind_of(*hash) == Kind::Point
}

/// Whether `hash` is a canonical Morton word.
///
/// Encoding zero-fills every bit below a cell's depth, so each cell has
/// exactly one bit pattern; a word that decodes but carries junk in those
/// bits is *not* canonical (and would break id equality). Both area words
/// and max-encoded point words can be canonical.
pub fn is_canonical(hash: &u64) -> bool {
    use mortie_core::decimal_morton::{Kind, from_nested_point, kind_of};

    match mortie_core::to_nested(*hash) {
        None => false,
        Some((depth, nested)) => match kind_of(*hash) {
            Kind::Area => mortie_core::from_nested(nested, depth) == *hash,
            Kind::Point => from_nested_point(nested) == *hash,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_nested_golden() {
        // words derived from the packed-word layout documented in
        // mortie-core ([4-bit prefix][27×2-bit body][6-bit suffix])
        assert_eq!(from_nested(&0, &0), 1152921504606846976);
        assert_eq!(from_nested(&0, &2), 1152921504606846978);
        assert_eq!(from_nested(&164, &3), 4107282860161892355);
        assert_eq!(from_nested(&3071, &4), 14983475960261640196);
        assert_eq!(from_nested(&11, &0), 13835058055282163712);
    }

    /// The low `2 * depth` bits of an arbitrary nonzero tuple pattern.
    ///
    /// The suffix packs the two deepest tuples into a single number
    /// (`t28 * 5 + t29 + 1`), so a hash whose tuples are all zero exercises
    /// none of that arithmetic.
    fn tail(depth: u8) -> u64 {
        0xC9B7 & ((1u64 << (2 * u32::from(depth))) - 1)
    }

    #[test]
    fn test_nested_round_trip() {
        for depth in [0u8, 1, 3, 13, 27, 28, 29] {
            for base in 0u64..12 {
                let first = base << (2 * depth as u32); // first cell of the base
                for hash in [first, first | tail(depth)] {
                    let word = from_nested(&hash, &depth);

                    assert_eq!(to_nested(&word), Some((hash, depth)));
                    assert!(is_canonical(&word));
                }
            }
        }
    }

    #[test]
    fn test_the_deepest_two_tuples_are_not_interchangeable() {
        // swapping t28 and t29 has to change the word — the round trips above
        // would pass either way if the tuples were always zero
        let base = 2u64 << (2 * 29);
        let (a, b) = (base | 0b01_10, base | 0b10_01);

        assert_ne!(from_nested(&a, &29), from_nested(&b, &29));
        assert_eq!(to_nested(&from_nested(&a, &29)), Some((a, 29)));
        assert_eq!(to_nested(&from_nested(&b, &29)), Some((b, 29)));
    }

    /// A coarse cell plus two deep ones carrying a nonzero suffix tail.
    fn crossing_cases() -> [(u64, u8); 3] {
        [
            (164, 3),
            ((2u64 << (2 * 28)) | tail(28), 28),
            ((2u64 << (2 * 29)) | tail(29), 29),
        ]
    }

    #[test]
    fn test_ring_round_trip() {
        for (hash, depth) in crossing_cases() {
            let ring = healpix::nested::get(depth).to_ring(hash);

            let word = from_ring(&ring, &depth);
            assert_eq!(word, from_nested(&hash, &depth));
            assert_eq!(to_ring(&word), Some((ring, depth)));
        }
    }

    #[test]
    fn test_zuniq_round_trip() {
        for (hash, depth) in crossing_cases() {
            let zuniq = healpix::nested::to_zuniq(depth, hash);

            let word = from_zuniq(&zuniq).unwrap();
            assert_eq!(word, from_nested(&hash, &depth));
            assert_eq!(to_zuniq(&word), Some(zuniq));
        }
    }

    #[test]
    fn test_invalid_zuniq_ids_do_not_re_encode() {
        // these would panic inside `cdshealpix` if passed straight through
        assert_eq!(from_zuniq(&0), None);
        assert_eq!(from_zuniq(&u64::MAX), None);
        assert_eq!(from_zuniq(&2), None); // sentinel bit at an odd position
    }

    #[test]
    fn test_invalid_words_do_not_decode() {
        // the empty sentinel and an invalid base-cell prefix (15)
        assert_eq!(to_nested(&0), None);
        assert_eq!(to_nested(&u64::MAX), None);
        assert!(!is_canonical(&0));
        assert!(!is_canonical(&u64::MAX));
    }

    #[test]
    fn test_non_canonical_words_are_rejected() {
        // a depth-3 word with junk below its depth decodes to the same cell
        // but is not the canonical bit pattern
        let word = from_nested(&164, &3);
        let junk = word | (1 << 30);

        assert_eq!(to_nested(&junk), to_nested(&word));
        assert!(is_canonical(&word));
        assert!(!is_canonical(&junk));
    }

    #[test]
    fn test_point_words_are_canonical() {
        let point = from_nested_point(&(164u64 << (2 * 26)));
        assert_eq!(point, 4107282860161892400);
        assert!(is_canonical(&point));
        assert_eq!(to_nested(&point), Some((164u64 << (2 * 26), 29)));
    }

    #[test]
    fn test_is_point_separates_the_kinds() {
        let nested = 164u64 << (2 * 26);
        assert!(is_point(&from_nested_point(&nested)));
        assert!(!is_point(&from_nested(&nested, &29)));
        assert!(!is_point(&from_nested(&164, &3)));
    }
}
