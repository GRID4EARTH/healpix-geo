use cdshealpix as healpix;

/// The deepest refinement level a `zuniq` id (or a Morton word) can encode.
const MAX_DEPTH: u8 = 29;

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
/// Returns `None` if `hash` is not a valid `zuniq` cell id. Unlike its
/// siblings this one checks its input: `cdshealpix`'s `from_zuniq` is
/// infallible in the type system but panics for `0` (the depth computation
/// underflows) and for an id whose cell index is out of range at the encoded
/// depth (`u64::MAX`, for instance), and it silently decodes an id whose
/// sentinel bit sits at an odd position (`2`, say) as cell `0` at depth 29 —
/// which would re-encode to a perfectly canonical Morton word for the wrong
/// cell. The bindings validate zuniq ids before converting, but a wrong cell
/// id fails silently, so the check is kept here as well.
pub fn to_morton(hash: &u64) -> Option<u64> {
    // a well-formed zuniq id has its sentinel bit at 2·(29 − depth); note
    // that `0` has 64 trailing zeros and is rejected by the upper bound
    let trailing = hash.trailing_zeros();
    if !trailing.is_multiple_of(2) || trailing > 2 * u32::from(MAX_DEPTH) {
        return None;
    }

    let (hash_nested, depth) = to_nested(hash);
    if hash_nested >= 12u64 << (2 * depth) {
        return None;
    }

    Some(mortie_core::from_nested(hash_nested, depth))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cdshealpix as healpix;

    #[test]
    fn test_to_morton_agrees_with_the_nested_encoder() {
        for (nested, depth) in [(164u64, 3u8), (5, 0), (2u64 << (2 * 29), 29)] {
            let zuniq = healpix::nested::to_zuniq(depth, nested);

            assert_eq!(
                to_morton(&zuniq),
                Some(crate::scalar::nested::conversion::to_morton(
                    &nested, &depth
                ))
            );
        }
    }

    #[test]
    fn test_invalid_zuniq_ids_do_not_re_encode() {
        // `0` and `u64::MAX` would panic inside `cdshealpix` if passed straight
        // through; `2` would decode as cell 0 at depth 29 and re-encode as the
        // canonical word for that cell
        assert_eq!(to_morton(&0), None);
        assert_eq!(to_morton(&u64::MAX), None);
        assert_eq!(to_morton(&2), None); // sentinel bit at an odd position
    }
}
