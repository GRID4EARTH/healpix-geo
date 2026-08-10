use cdshealpix as healpix;
use geodesy::prelude::EllipsoidBase;
use healpix_geo_core::ellipsoid::{Ellipsoid as RustEllipsoid, ReferenceBody};
use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

use crate::coordinates::Coordinate;
use crate::ellipsoid::EllipsoidLike;
use crate::geometry::spherical_vertex;

const MAX_LEVEL: u8 = 29;

/// Number of cells of a full HEALPix grid at `level` (12 · 4^level).
fn n_cells(level: u8) -> u64 {
    12u64 << (2 * level)
}

/// Decode a `zuniq` cell id.
///
/// `cdshealpix`'s `from_zuniq` is infallible in the type system but panics —
/// an unrecoverable wasm trap — for values that are not valid zuniq
/// encodings: `0` underflows the level computation, and ids whose sentinel bit
/// sits at an odd position or whose hash is out of range at the encoded level
/// (`u64::MAX`, for instance) blow up further down. Validate first.
fn from_zuniq_checked(cell: u64) -> Result<(u8, u64), String> {
    // a well-formed zuniq id has its sentinel bit at 2·(29 − level); note
    // that `0` has 64 trailing zeros and is rejected by the upper bound
    let trailing = cell.trailing_zeros();
    if !trailing.is_multiple_of(2) || trailing > 2 * u32::from(MAX_LEVEL) {
        return Err(format!("{} is not a valid zuniq cell id", cell));
    }

    let (level, hash) = healpix::nested::from_zuniq(cell);
    if hash >= n_cells(level) {
        return Err(format!(
            "{} is not a valid zuniq cell id: hash {} is out of range at level {}",
            cell, hash, level
        ));
    }

    Ok((level, hash))
}

/// Narrow a JS `number` to a `u32`, rejecting what wasm-bindgen would coerce.
///
/// wasm-bindgen converts a `number` parameter declared as `u32` with
/// JavaScript's ToUint32 semantics — truncate the fraction, then take the
/// result modulo 2³² — *before* Rust sees it. `bitCombine(2 ** 32, 0)` would
/// arrive as `(0, 0)` and quietly pass the range check it was meant to fail,
/// so the count-like parameters are taken as `f64` and narrowed here instead.
/// The generated TypeScript signature is `number` either way.
fn to_u32(value: f64, name: &str) -> Result<u32, String> {
    if !value.is_finite() || value.fract() != 0.0 {
        return Err(format!("`{}` must be an integer, got {}", name, value));
    }
    if !(0.0..=f64::from(u32::MAX)).contains(&value) {
        return Err(format!(
            "`{}` must be in [0, {}], got {}",
            name,
            u32::MAX,
            value
        ));
    }

    Ok(value as u32)
}

/// Narrow a JS `number` to a refinement level.
///
/// Range-checked against the level bound directly instead of narrowing through
/// `to_u32` first, so a negative level is reported as out of `[0, 29]` rather
/// than out of the `u32` range, which is not this parameter's contract.
fn to_level(value: f64) -> Result<u8, String> {
    if !value.is_finite() || value.fract() != 0.0 {
        return Err(format!("`level` must be an integer, got {}", value));
    }
    if !(0.0..=f64::from(MAX_LEVEL)).contains(&value) {
        return Err(format!(
            "`level` must be in [0, {}], got {}",
            MAX_LEVEL, value
        ));
    }

    Ok(value as u8)
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Scheme {
    Nested,
    Ring,
    Zuniq,
}

impl Scheme {
    fn name(&self) -> &'static str {
        match self {
            Self::Nested => "nested",
            Self::Ring => "ring",
            Self::Zuniq => "zuniq",
        }
    }

    fn parse(name: &str) -> Result<Scheme, String> {
        match name {
            "nested" => Ok(Self::Nested),
            "ring" => Ok(Self::Ring),
            "zuniq" => Ok(Self::Zuniq),
            _ => Err(format!(
                "unknown scheme {:?}: expected \"nested\", \"ring\" or \"zuniq\"",
                name
            )),
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct GridOptions {
    scheme: Scheme,
    level: u8,
    #[serde(default)]
    ellipsoid: Option<EllipsoidLike>,
}

/// The options object accepted by the `Grid` constructor.
#[wasm_bindgen(typescript_custom_section)]
const GRID_OPTIONS: &'static str = r#"
export type GridOptions = {
    scheme: "nested" | "ring" | "zuniq";
    level: number;
    ellipsoid?: EllipsoidInput | null;
};
"#;

/// A HEALPix grid at a fixed refinement level on a fixed reference body.
///
/// Constructed with `new Grid(options)`. The scheme dispatch, the level, and
/// the parsed ellipsoid state (including the authalic-latitude coefficients)
/// are stored once, so per-call overhead is limited to the actual coordinate
/// math.
///
/// Every method validates its inputs and reports misuse as a catchable JS
/// `Error` rather than trapping the wasm instance.
#[wasm_bindgen]
pub struct Grid {
    scheme: Scheme,
    level: u8,
    pub(crate) ellipsoid: RustEllipsoid,
}

impl Grid {
    pub(crate) fn from_options(options: GridOptions) -> Result<Grid, String> {
        let level = options.level;
        if level > MAX_LEVEL {
            return Err(format!(
                "`level` must be in [0, {}], got {}",
                MAX_LEVEL, level
            ));
        }

        let ellipsoid = options
            .ellipsoid
            .map(|e| e.into_ellipsoid())
            .transpose()?
            .unwrap_or_default();

        Ok(Grid {
            scheme: options.scheme,
            level,
            ellipsoid,
        })
    }

    /// Number of cells at the grid's level.
    fn n_cells(&self) -> u64 {
        n_cells(self.level)
    }

    fn check_cell(&self, cell: u64) -> Result<(), String> {
        if cell >= self.n_cells() {
            return Err(format!(
                "cell id {} is out of range at level {} ({} cells)",
                cell,
                self.level,
                self.n_cells()
            ));
        }

        Ok(())
    }

    /// The cell in the nested scheme plus the level it lives at.
    ///
    /// The `ring` branch converts to nested and then uses the nested
    /// coordinate implementation (the same route
    /// [`healpix_geo_wasm::ring`][crate::ring] takes), rather than
    /// `healpix_geo_core::scalar::ring::coordinates`; the two agree
    /// everywhere tested, but only one of them is exercised.
    pub(crate) fn to_nested(&self, cell: u64) -> Result<(u8, u64), String> {
        match self.scheme {
            Scheme::Nested => {
                self.check_cell(cell)?;
                Ok((self.level, cell))
            }
            Scheme::Ring => {
                self.check_cell(cell)?;
                Ok((self.level, healpix::nested::get(self.level).from_ring(cell)))
            }
            Scheme::Zuniq => from_zuniq_checked(cell),
        }
    }

    /// A nested hash at the grid's level, expressed in the grid's scheme.
    fn nested_to_scheme(&self, hash: u64) -> u64 {
        match self.scheme {
            Scheme::Nested => hash,
            Scheme::Ring => healpix::nested::get(self.level).to_ring(hash),
            Scheme::Zuniq => healpix::nested::to_zuniq(self.level, hash),
        }
    }

    /// Convert a cell id from the grid's scheme to `target`.
    ///
    /// Without `level`: `nested` ↔ `ring` convert at the grid's level;
    /// converting *to* `zuniq` encodes the grid's level; converting *from*
    /// `zuniq` uses the level embedded in the id (as [`Grid::vertex_impl`]
    /// does), so the result lives at that level and `zuniq` → `zuniq` is the
    /// identity.
    ///
    /// With `level` (mirroring the Python bindings' `auto.convert`): only
    /// valid when `target` encodes the level in its cell ids — `zuniq` today —
    /// and the grid's scheme does not; `cell` is then read as a cell of the
    /// grid's scheme at `level` and encoded with it.
    pub(crate) fn to_scheme_impl(
        &self,
        cell: u64,
        target: Scheme,
        level: Option<f64>,
    ) -> Result<u64, String> {
        let Some(level) = level else {
            let (level, hash) = self.to_nested(cell)?;

            return Ok(match target {
                Scheme::Nested => hash,
                Scheme::Ring => healpix::nested::get(level).to_ring(hash),
                Scheme::Zuniq => healpix::nested::to_zuniq(level, hash),
            });
        };

        if self.scheme == Scheme::Zuniq {
            return Err(
                "`level` is invalid when converting from \"zuniq\": the cell ids already encode their level"
                    .to_string(),
            );
        }
        if target != Scheme::Zuniq {
            return Err(format!(
                "`level` is only valid when converting to a scheme that encodes the level in its cell ids (\"zuniq\"), not \"{}\"",
                target.name()
            ));
        }

        let level = to_level(level)?;
        if cell >= n_cells(level) {
            return Err(format!(
                "cell id {} is out of range at level {} ({} cells)",
                cell,
                level,
                n_cells(level)
            ));
        }

        let hash = match self.scheme {
            Scheme::Nested => cell,
            Scheme::Ring => healpix::nested::get(level).from_ring(cell),
            Scheme::Zuniq => unreachable!("rejected above"),
        };

        Ok(healpix::nested::to_zuniq(level, hash))
    }

    pub(crate) fn vertex_impl(&self, cell: u64, u: f64, v: f64) -> Result<Coordinate, String> {
        let (level, hash) = self.to_nested(cell)?;
        let layer = healpix::nested::get(level);

        let center = layer.center_of_projected_cell(hash);
        let (lon, lat) = spherical_vertex(center, level, (u, v))?;

        Ok(Coordinate {
            lon: lon.to_degrees().rem_euclid(360.0),
            lat: self
                .ellipsoid
                .latitude_authalic_to_geographic(lat)
                .to_degrees(),
        })
    }

    pub(crate) fn bit_combine_impl(&self, i: f64, j: f64) -> Result<u64, String> {
        let (i, j) = (to_u32(i, "i")?, to_u32(j, "j")?);

        let nside = self.nside();
        if i >= nside || j >= nside {
            return Err(format!(
                "z-order coordinates ({}, {}) are out of range for nside {}",
                i, j, nside
            ));
        }

        let zoc = healpix::nested::zordercurve::get_zoc(self.level);

        Ok(self.nested_to_scheme(zoc.ij2h(i, j)))
    }
}

#[wasm_bindgen]
impl Grid {
    /// Create a grid from plain options.
    ///
    /// Options:
    /// - `scheme`: `"nested"`, `"ring"` or `"zuniq"` (required)
    /// - `level`: the refinement level, at most 29; level 0 is the 12 base
    ///   cells (required)
    /// - `ellipsoid`: a plain object as accepted by `Ellipsoid.from`, or
    ///   `null`/absent for the default sphere
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(unchecked_param_type = "GridOptions")] options: JsValue,
    ) -> Result<Grid, JsValue> {
        let options: GridOptions = from_value(options)?;

        Grid::from_options(options).map_err(|message| JsError::new(&message).into())
    }

    /// The indexing scheme: "nested", "ring" or "zuniq"
    ///
    /// Narrowed to the literal union so that `other.toScheme(cell,
    /// grid.scheme)` type-checks and `switch (grid.scheme)` is exhaustive.
    #[wasm_bindgen(getter, unchecked_return_type = "\"nested\" | \"ring\" | \"zuniq\"")]
    pub fn scheme(&self) -> String {
        self.scheme.name().to_string()
    }

    /// The refinement level of the grid (0-indexed; level 0 is the 12 base
    /// cells)
    #[wasm_bindgen(getter)]
    pub fn level(&self) -> u8 {
        self.level
    }

    /// The nside (2^level) of the grid
    #[wasm_bindgen(getter)]
    pub fn nside(&self) -> u32 {
        1u32 << self.level
    }

    /// Semi-major axis of the reference body (the radius, for a sphere), in
    /// meters
    #[wasm_bindgen(getter, js_name = semiMajorAxis)]
    pub fn semi_major_axis(&self) -> f64 {
        self.ellipsoid.ellipsoid().semimajor_axis()
    }

    /// Flattening of the reference body (0 for a sphere)
    #[wasm_bindgen(getter)]
    pub fn flattening(&self) -> f64 {
        self.ellipsoid.ellipsoid().flattening()
    }

    /// Whether the reference body is a sphere
    #[wasm_bindgen(getter, js_name = isSphere)]
    pub fn is_sphere(&self) -> bool {
        matches!(self.ellipsoid, RustEllipsoid::Sphere(_))
    }

    /// The reference body of the grid, as a **new** handle
    ///
    /// Every read clones the parsed ellipsoid into a freshly allocated
    /// `Ellipsoid` (wasm allocation + JS wrapper + finalizer registration), so
    /// this is not a free property access: hoist it out of hot paths and
    /// `free()` it when done, or use the `semiMajorAxis` / `flattening` /
    /// `isSphere` getters above, which return plain numbers.
    #[wasm_bindgen(getter)]
    pub fn ellipsoid(&self) -> crate::ellipsoid::Ellipsoid {
        crate::ellipsoid::Ellipsoid {
            inner: self.ellipsoid.clone(),
        }
    }

    /// Single vertex of the given cell
    ///
    /// `u` and `v` are offsets from the southern vertex of the cell, in
    /// `[0, 1]`; anything else throws. For the `zuniq` scheme, the level
    /// encoded in the cell id is used.
    pub fn vertex(&self, cell: u64, u: f64, v: f64) -> Result<Coordinate, JsValue> {
        self.vertex_impl(cell, u, v)
            .map_err(|message| JsError::new(&message).into())
    }

    /// Cell index at the given z-order coordinates
    ///
    /// Interleaves the bits of `i` and `j` (the two axes of the nested
    /// z-order numbering within a base-resolution pixel) into the cell index
    /// at the grid's level, expressed in the grid's scheme. Both coordinates
    /// have to be non-negative integers smaller than `nside`.
    #[wasm_bindgen(js_name = bitCombine)]
    pub fn bit_combine(&self, i: f64, j: f64) -> Result<u64, JsValue> {
        self.bit_combine_impl(i, j)
            .map_err(|message| JsError::new(&message).into())
    }

    /// Convert a cell id from the grid's scheme to another scheme
    ///
    /// `scheme` is one of `"nested"`, `"ring"` or `"zuniq"`. `nested` and
    /// `ring` convert at the grid's level. Converting to `zuniq` encodes the
    /// grid's level; converting from `zuniq` uses the level encoded in the
    /// cell id (as `vertex` does), so the result lives at that level and
    /// `zuniq` to `zuniq` is the identity.
    ///
    /// The optional `level` overrides the level `cell` is read and encoded
    /// at. It is only valid when converting to a scheme that encodes the
    /// level in its cell ids — `"zuniq"` today — and from a scheme that does
    /// not (`nested` or `ring`); anything else throws.
    #[wasm_bindgen(js_name = toScheme)]
    pub fn to_scheme(
        &self,
        cell: u64,
        #[wasm_bindgen(unchecked_param_type = "\"nested\" | \"ring\" | \"zuniq\"")] scheme: &str,
        level: Option<f64>,
    ) -> Result<u64, JsValue> {
        Scheme::parse(scheme)
            .and_then(|target| self.to_scheme_impl(cell, target, level))
            .map_err(|message| JsError::new(&message).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options(scheme: Scheme, level: u8) -> GridOptions {
        GridOptions {
            scheme,
            level,
            ellipsoid: None,
        }
    }

    fn grid(scheme: Scheme, level: u8) -> Grid {
        Grid::from_options(options(scheme, level)).unwrap()
    }

    #[test]
    fn test_level_validation() {
        assert!(Grid::from_options(options(Scheme::Nested, 30)).is_err());

        let valid = Grid::from_options(options(Scheme::Nested, 3)).unwrap();
        assert_eq!(valid.level(), 3);
        assert_eq!(valid.nside(), 8);
    }

    #[test]
    fn test_vertex_matches_scheme_statics() {
        let grid = grid(Scheme::Nested, 0);

        let expected: Vec<(f64, f64)> = vec![
            (45.0, 0.0),
            (67.5, 19.47122063),
            (90.0, 41.8103149),
            (45.0, 90.0),
        ];
        let uv: Vec<(f64, f64)> = vec![(0.0, 0.0), (0.5, 0.0), (1.0, 0.0), (1.0, 1.0)];

        for ((u, v), (lon, lat)) in uv.into_iter().zip(expected) {
            let actual = grid.vertex_impl(0, u, v).unwrap();
            assert!((actual.lon - lon).abs() < 1e-4);
            assert!((actual.lat - lat).abs() < 1e-4);
        }
    }

    #[test]
    fn test_zuniq_uses_encoded_level() {
        // level 0, nested cell 0 encoded as zuniq
        let cell = healpix::nested::to_zuniq(0, 0);
        let grid = grid(Scheme::Zuniq, 4);

        let vertex = grid.vertex_impl(cell, 0.0, 0.0).unwrap();
        assert!((vertex.lon - 45.0).abs() < 1e-4);
        assert!(vertex.lat.abs() < 1e-4);
    }

    #[test]
    fn test_flattened_ellipsoid_getters_match_the_handle() {
        let grid = grid(Scheme::Nested, 4);

        assert_eq!(grid.semi_major_axis(), grid.ellipsoid().semi_major_axis());
        assert_eq!(grid.flattening(), grid.ellipsoid().flattening());
        assert_eq!(grid.is_sphere(), grid.ellipsoid().is_sphere());
        assert!(grid.is_sphere());
    }

    #[test]
    fn test_rejects_out_of_range_cells() {
        // 12 · 4^4 == 3072 is the first invalid id at level 4; cdshealpix
        // panics on it ("Wrong hash value: too large")
        for scheme in [Scheme::Nested, Scheme::Ring] {
            let grid = grid(scheme, 4);
            assert_eq!(grid.n_cells(), 3072);

            assert!(grid.vertex_impl(3072, 0.5, 0.5).is_err());
            assert!(grid.vertex_impl(u64::MAX, 0.5, 0.5).is_err());
            assert!(grid.vertex_impl(3071, 0.5, 0.5).is_ok());
        }
    }

    #[test]
    fn test_rejects_invalid_zuniq_cells() {
        let grid = grid(Scheme::Zuniq, 4);

        // `0` used to underflow the level computation
        assert!(grid.vertex_impl(0, 0.5, 0.5).is_err());
        // sentinel bit at an odd position
        assert!(grid.vertex_impl(2, 0.5, 0.5).is_err());
        // hash out of range at the encoded level
        assert!(grid.vertex_impl(u64::MAX, 0.5, 0.5).is_err());

        let valid = healpix::nested::to_zuniq(4, 164);
        assert!(grid.vertex_impl(valid, 0.5, 0.5).is_ok());
    }

    #[test]
    fn test_bit_combine_rejects_out_of_range_coordinates() {
        // ring is the dangerous one: an out-of-range nested hash used to
        // panic inside `to_ring` ("assertion failed: j_d0h <= 2")
        for scheme in [Scheme::Nested, Scheme::Ring, Scheme::Zuniq] {
            let grid = grid(scheme, 1);
            assert_eq!(grid.nside(), 2);

            assert!(grid.bit_combine_impl(0.0, 2.0).is_err());
            assert!(grid.bit_combine_impl(256.0, 0.0).is_err());
            assert!(grid.bit_combine_impl(1.0, 1.0).is_ok());
        }
    }

    #[test]
    fn test_bit_combine_rejects_non_integer_and_wrapping_coordinates() {
        // wasm-bindgen would have coerced each of these to an in-range `u32`
        // before the guard above ever ran: `2^32` to 0, `1.9` to 1, `NaN` to 0
        let grid = grid(Scheme::Nested, 4);

        assert!(grid.bit_combine_impl(4294967296.0, 0.0).is_err());
        assert!(grid.bit_combine_impl(1.9, 1.0).is_err());
        assert!(grid.bit_combine_impl(f64::NAN, 0.0).is_err());
        assert!(grid.bit_combine_impl(f64::INFINITY, 0.0).is_err());
        assert!(grid.bit_combine_impl(-1.0, 0.0).is_err());
        assert!(grid.bit_combine_impl(0.0, -1.0).is_err());

        assert!(grid.bit_combine_impl(1.0, 1.0).is_ok());
    }

    #[test]
    fn test_vertex_rejects_out_of_range_offsets() {
        // `u`/`v` outside `[0, 1]` used to trap the wasm instance inside
        // `unproj`'s `-2 <= y <= 2` assertion once they left the projection
        // plane; the contract is now a strict `[0, 1]`
        let grid = grid(Scheme::Nested, 4);

        assert!(grid.vertex_impl(164, f64::NAN, 0.0).is_err());
        assert!(grid.vertex_impl(164, 0.0, f64::NAN).is_err());
        assert!(grid.vertex_impl(164, f64::INFINITY, 0.0).is_err());
        assert!(grid.vertex_impl(164, -0.1, 0.0).is_err());
        assert!(grid.vertex_impl(164, 0.0, 1.5).is_err());
        assert!(grid.vertex_impl(164, 10.0, 0.0).is_err());

        // the whole cell-local range stays legal, boundaries included
        assert!(grid.vertex_impl(164, 0.0, 0.0).is_ok());
        assert!(grid.vertex_impl(164, 0.5, 0.5).is_ok());
        assert!(grid.vertex_impl(164, 1.0, 1.0).is_ok());
    }

    #[test]
    fn test_bit_combine_matches_statics() {
        assert_eq!(
            grid(Scheme::Zuniq, 1).bit_combine_impl(0.0, 1.0).unwrap(),
            360287970189639680
        );
        assert_eq!(grid(Scheme::Ring, 1).bit_combine_impl(0.0, 1.0).unwrap(), 4);
        assert_eq!(
            grid(Scheme::Nested, 1).bit_combine_impl(0.0, 1.0).unwrap(),
            2
        );
    }

    #[test]
    fn test_to_scheme_nested_ring_roundtrip() {
        // at level 1, nested 2 == ring 4 (see test_bit_combine_matches_statics)
        let nested = grid(Scheme::Nested, 1);
        let ring = grid(Scheme::Ring, 1);

        assert_eq!(nested.to_scheme_impl(2, Scheme::Ring, None).unwrap(), 4);
        assert_eq!(ring.to_scheme_impl(4, Scheme::Nested, None).unwrap(), 2);

        // same-scheme conversions are the identity
        assert_eq!(nested.to_scheme_impl(2, Scheme::Nested, None).unwrap(), 2);
        assert_eq!(ring.to_scheme_impl(4, Scheme::Ring, None).unwrap(), 4);
    }

    #[test]
    fn test_to_scheme_zuniq_encodes_the_grid_level() {
        let nested = grid(Scheme::Nested, 4);
        let id = nested.to_scheme_impl(164, Scheme::Zuniq, None).unwrap();
        assert_eq!(id, healpix::nested::to_zuniq(4, 164));

        let zuniq = grid(Scheme::Zuniq, 4);
        assert_eq!(zuniq.to_scheme_impl(id, Scheme::Nested, None).unwrap(), 164);
    }

    #[test]
    fn test_to_scheme_zuniq_uses_the_embedded_level() {
        // a level-0 id in a level-4 grid converts at level 0, mirroring how
        // `vertex` treats zuniq ids
        let zuniq = grid(Scheme::Zuniq, 4);
        let cell = healpix::nested::to_zuniq(0, 5);

        assert_eq!(zuniq.to_scheme_impl(cell, Scheme::Nested, None).unwrap(), 5);
        assert_eq!(
            zuniq.to_scheme_impl(cell, Scheme::Ring, None).unwrap(),
            healpix::nested::get(0).to_ring(5)
        );
        // zuniq -> zuniq keeps the id as-is
        assert_eq!(
            zuniq.to_scheme_impl(cell, Scheme::Zuniq, None).unwrap(),
            cell
        );
    }

    #[test]
    fn test_to_scheme_level_override() {
        let nested = grid(Scheme::Nested, 4);

        // the grid's own level is a valid override
        assert_eq!(
            nested
                .to_scheme_impl(164, Scheme::Zuniq, Some(4.0))
                .unwrap(),
            nested.to_scheme_impl(164, Scheme::Zuniq, None).unwrap()
        );
        // a coarser override reads (and encodes) the id at that level
        assert_eq!(
            nested.to_scheme_impl(5, Scheme::Zuniq, Some(0.0)).unwrap(),
            healpix::nested::to_zuniq(0, 5)
        );
        // ring ids are converted at the override level too: ring 4 == nested
        // 2 at level 1
        let ring = grid(Scheme::Ring, 4);
        assert_eq!(
            ring.to_scheme_impl(4, Scheme::Zuniq, Some(1.0)).unwrap(),
            healpix::nested::to_zuniq(1, 2)
        );
        // and the cell id is validated against the override level
        assert!(
            nested
                .to_scheme_impl(164, Scheme::Zuniq, Some(0.0))
                .is_err()
        );
    }

    #[test]
    fn test_to_scheme_level_rejections() {
        let nested = grid(Scheme::Nested, 4);

        // schemes whose ids do not encode the level reject the override
        assert!(nested.to_scheme_impl(164, Scheme::Ring, Some(4.0)).is_err());
        assert!(
            nested
                .to_scheme_impl(164, Scheme::Nested, Some(4.0))
                .is_err()
        );

        // zuniq ids already carry their level, even for the identity
        let zuniq = grid(Scheme::Zuniq, 4);
        let id = healpix::nested::to_zuniq(4, 164);
        assert!(zuniq.to_scheme_impl(id, Scheme::Zuniq, Some(4.0)).is_err());
        assert!(zuniq.to_scheme_impl(id, Scheme::Nested, Some(4.0)).is_err());

        // and the level itself is validated
        assert!(
            nested
                .to_scheme_impl(164, Scheme::Zuniq, Some(30.0))
                .is_err()
        );
        assert!(
            nested
                .to_scheme_impl(164, Scheme::Zuniq, Some(2.5))
                .is_err()
        );
        assert!(
            nested
                .to_scheme_impl(164, Scheme::Zuniq, Some(f64::NAN))
                .is_err()
        );
        assert!(
            nested
                .to_scheme_impl(164, Scheme::Zuniq, Some(-1.0))
                .is_err()
        );
    }

    #[test]
    fn test_to_scheme_validates_the_input_id() {
        assert!(
            grid(Scheme::Nested, 4)
                .to_scheme_impl(3072, Scheme::Ring, None)
                .is_err()
        );
        assert!(
            grid(Scheme::Zuniq, 4)
                .to_scheme_impl(0, Scheme::Nested, None)
                .is_err()
        );
        assert!(Scheme::parse("bogus").is_err());
    }
}
