use healpix_geo::scalar::morton::{conversion, hierarchy};
use wasm_bindgen::prelude::*;

use crate::grid::to_level;

/// Decode any canonical `morton` word, area or max-encoded point.
///
/// `mortie-core` zero-fills every bit below a cell's level, so each cell has
/// exactly one bit pattern; a word carrying junk in those bits would silently
/// alias another id of the same cell and is rejected here.
pub(crate) fn morton_word_checked(cell: u64) -> Result<(u8, u64), String> {
    if !conversion::is_canonical(&cell) {
        return Err(format!("{} is not a valid morton cell id", cell));
    }

    // is_canonical implies the word decodes
    let (hash, level) = conversion::to_nested(&cell).unwrap();

    Ok((level, hash))
}

/// Decode a `morton` *cell* id: a canonical area word.
///
/// Max-encoded point words (a coordinate cast to level 29 with no area claim)
/// are not cell ids: a point and the level-29 area cell covering it are two
/// distinct canonical words that decode to the same `(level, hash)`, so
/// nothing downstream of this function can tell them apart and no conversion
/// round-trip can preserve them. A point also claims no area, which is what
/// `vertex`/`vertices` would need. The `morton` statics, which only read the
/// word, take them through [`morton_word_checked`] instead.
pub(crate) fn from_morton_checked(cell: u64) -> Result<(u8, u64), String> {
    let (level, hash) = morton_word_checked(cell)?;

    if conversion::is_point(&cell) {
        return Err(format!(
            "{} is a max-encoded point word, not a morton cell id",
            cell
        ));
    }

    Ok((level, hash))
}

/// Codec-only operations on `morton` cell ids.
///
/// A `morton` id is a packed-`u64` Morton word (the `mortie-core` codec): it
/// encodes its own refinement level, a raw unsigned sort of *area* words is a
/// Z-order traversal with parents immediately before their children, and
/// containment testing is plain prefix truncation. None of these operations
/// need a reference body, so they live here rather than on `Grid`; coordinate
/// math on morton ids goes through `new Grid({ scheme: "morton", ... })`.
///
/// These statics take **any** canonical word — area or max-encoded point (a
/// coordinate cast to level 29 with no area claim). A point has a level (29),
/// coarsens to the area cell containing it, and is contained by that cell and
/// its ancestors; it contains nothing itself, so `contains` answers `false`
/// for a point ancestor rather than throwing. It is outside the sort order
/// above, though: a point sorts after the entire level-28/29 area region of
/// its body, so it is not part of any subtree run. `Grid` is the exception:
/// it takes area words only, since a point has no area to draw or convert.
#[wasm_bindgen(js_name = morton)]
pub struct Morton;

impl Morton {
    pub(crate) fn level_impl(cell: u64) -> Result<u8, String> {
        let (level, _) = morton_word_checked(cell)?;

        Ok(level)
    }

    pub(crate) fn ancestor_impl(cell: u64, level: f64) -> Result<u64, String> {
        morton_word_checked(cell)?;
        let level = to_level(level)?;

        // the id decodes (checked above), so `ancestor` cannot return `None`
        Ok(hierarchy::ancestor(&cell, &level).unwrap())
    }

    pub(crate) fn contains_impl(ancestor: u64, descendant: u64) -> Result<bool, String> {
        morton_word_checked(ancestor)?;
        morton_word_checked(descendant)?;

        Ok(hierarchy::contains(&ancestor, &descendant))
    }

    pub(crate) fn is_point_impl(cell: u64) -> Result<bool, String> {
        morton_word_checked(cell)?;

        Ok(conversion::is_point(&cell))
    }
}

#[wasm_bindgen(js_class = morton)]
impl Morton {
    /// The refinement level encoded in the given cell id
    ///
    /// A max-encoded point word reports 29, the level it was cast to.
    #[wasm_bindgen(js_name = level)]
    pub fn level(cell: u64) -> Result<u8, JsValue> {
        Morton::level_impl(cell).map_err(|message| JsError::new(&message).into())
    }

    /// The ancestor of the given cell id at `level`
    ///
    /// Truncates the id to `level`, discarding finer detail; `level` is the
    /// level to land at, not a number of levels to drop, and a `level` at or
    /// above the id's own level returns the id unchanged. A point word
    /// coarsens to the area cell containing it, so it shares every ancestor
    /// with the level-29 cell it falls in.
    #[wasm_bindgen(js_name = ancestor)]
    pub fn ancestor(cell: u64, level: f64) -> Result<u64, JsValue> {
        Morton::ancestor_impl(cell, level).map_err(|message| JsError::new(&message).into())
    }

    /// Whether `ancestor` contains `descendant` (containment-is-truncation)
    ///
    /// True iff truncating `descendant` to `ancestor`'s level yields exactly
    /// `ancestor`; a cell contains itself, and a level-29 cell contains the
    /// point words falling inside it. A point claims no area, so a point
    /// `ancestor` is `false` — not an error. Both ids are validated first, so
    /// anything that is not a canonical word throws.
    #[wasm_bindgen(js_name = contains)]
    pub fn contains(ancestor: u64, descendant: u64) -> Result<bool, JsValue> {
        Morton::contains_impl(ancestor, descendant).map_err(|message| JsError::new(&message).into())
    }

    /// Whether the given id is a max-encoded point word rather than an area cell
    ///
    /// Throws on a non-canonical word, like the other statics.
    #[wasm_bindgen(js_name = isPoint)]
    pub fn is_point(cell: u64) -> Result<bool, JsValue> {
        Morton::is_point_impl(cell).map_err(|message| JsError::new(&message).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use healpix_geo::scalar::morton::conversion::{from_nested, from_nested_point};

    /// A point word and the level-29 area cell of the same body.
    fn point_and_area() -> (u64, u64) {
        let nested = 164u64 << (2 * 26);

        (from_nested_point(&nested), from_nested(&nested, &29))
    }

    #[test]
    fn test_level() {
        assert_eq!(Morton::level_impl(from_nested(&164, &3)).unwrap(), 3);
        assert_eq!(Morton::level_impl(from_nested(&2, &0)).unwrap(), 0);
        assert!(Morton::level_impl(0).is_err());
        assert!(Morton::level_impl(u64::MAX).is_err());
    }

    #[test]
    fn test_ancestor() {
        let cell = from_nested(&164, &3);

        assert_eq!(
            Morton::ancestor_impl(cell, 2.0).unwrap(),
            from_nested(&41, &2)
        );
        assert_eq!(Morton::ancestor_impl(cell, 3.0).unwrap(), cell);
        assert_eq!(Morton::ancestor_impl(cell, 29.0).unwrap(), cell);
        assert!(Morton::ancestor_impl(0, 2.0).is_err());
        assert!(Morton::ancestor_impl(cell, 2.5).is_err());
        assert!(Morton::ancestor_impl(cell, 30.0).is_err());
        assert!(Morton::ancestor_impl(cell, -1.0).is_err());
    }

    #[test]
    fn test_contains() {
        let child = from_nested(&164, &3);
        let parent = from_nested(&41, &2);

        assert!(Morton::contains_impl(parent, child).unwrap());
        assert!(!Morton::contains_impl(child, parent).unwrap());
        assert!(Morton::contains_impl(child, child).unwrap());
        assert!(Morton::contains_impl(0, child).is_err());
        assert!(Morton::contains_impl(child, u64::MAX).is_err());
    }

    #[test]
    fn test_level_reads_point_words() {
        let (point, area) = point_and_area();

        assert_eq!(Morton::level_impl(point).unwrap(), 29);
        assert_eq!(Morton::level_impl(area).unwrap(), 29);
    }

    #[test]
    fn test_ancestor_coarsens_a_point_to_its_cell() {
        let (point, area) = point_and_area();

        // below level 29 a point and the cell it falls in share every ancestor
        for level in [0u8, 3, 24, 28] {
            assert_eq!(
                Morton::ancestor_impl(point, f64::from(level)).unwrap(),
                Morton::ancestor_impl(area, f64::from(level)).unwrap()
            );
        }
        // at its own level (and above) the point is returned unchanged
        assert_eq!(Morton::ancestor_impl(point, 29.0).unwrap(), point);
    }

    #[test]
    fn test_contains_covers_point_words() {
        let (point, area) = point_and_area();

        assert!(Morton::contains_impl(area, point).unwrap());
        for level in [0u8, 3, 24, 28] {
            let covering = Morton::ancestor_impl(area, f64::from(level)).unwrap();
            assert!(Morton::contains_impl(covering, point).unwrap());
        }
        assert!(!Morton::contains_impl(from_nested(&165, &3), point).unwrap());

        // a point claims no area: `false`, not an error
        assert!(!Morton::contains_impl(point, point).unwrap());
        assert!(!Morton::contains_impl(point, area).unwrap());
    }

    #[test]
    fn test_is_point_separates_the_kinds() {
        let (point, area) = point_and_area();

        assert!(Morton::is_point_impl(point).unwrap());
        assert!(!Morton::is_point_impl(area).unwrap());
        assert!(!Morton::is_point_impl(from_nested(&164, &3)).unwrap());
        assert!(Morton::is_point_impl(0).is_err());
        assert!(Morton::is_point_impl(u64::MAX).is_err());
    }

    #[test]
    fn test_grid_still_rejects_point_words() {
        let (point, area) = point_and_area();

        assert!(from_morton_checked(point).is_err());
        assert_eq!(from_morton_checked(area).unwrap().0, 29);
    }
}
