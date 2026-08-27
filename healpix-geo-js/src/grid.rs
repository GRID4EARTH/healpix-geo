use cdshealpix as healpix;
use geodesy::prelude::EllipsoidBase;
use healpix_geo::ellipsoid::{Ellipsoid as RustEllipsoid, ReferenceBody};
use healpix_geo::scalar::nested::coordinates as scalar;
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

/// Capacity for `n` elements, or an error if that does not fit a `usize`.
///
/// `usize` is 32 bits on `wasm32`, and `steps`/`size` are `u32`, so the
/// element count has to be computed in `u64` and checked — otherwise release
/// builds get a wrapped capacity hint and debug builds panic on the multiply.
fn capacity(n: Option<u64>, what: &str) -> Result<usize, String> {
    n.and_then(|n| usize::try_from(n).ok())
        .ok_or_else(|| format!("{} is too large: the output array would not fit", what))
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
    /// `healpix_geo::scalar::ring::coordinates`; the two agree
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

    /// The cell containing `(lon, lat)`, as a nested hash at the grid's level.
    ///
    /// `cdshealpix`'s `Layer::hash` asserts `-π/2 ≤ lat ≤ π/2` and traps
    /// otherwise — including on `NaN`, which fails the comparison. A
    /// non-finite longitude survives `rem_euclid` as `NaN` and trips the same
    /// assert, so both are rejected up front.
    fn lonlat_to_nested(&self, lon: f64, lat: f64) -> Result<u64, String> {
        if !lon.is_finite() {
            return Err(format!("longitude must be a finite number, got {}", lon));
        }
        if !(-90.0..=90.0).contains(&lat) {
            return Err(format!("latitude must be in [-90, 90], got {}", lat));
        }

        let layer = healpix::nested::get(self.level);

        Ok(scalar::lonlat_to_healpix(
            &lon,
            &lat,
            layer,
            &self.ellipsoid,
        ))
    }

    pub(crate) fn vertices_impl(&self, cell: u64, steps: f64) -> Result<Vec<f64>, String> {
        let steps = to_u32(steps, "steps")?;
        if steps < 2 {
            return Err("`steps` must be at least 2".to_string());
        }
        let count = capacity(
            u64::from(steps)
                .checked_mul(u64::from(steps))
                .and_then(|n| n.checked_mul(2)),
            &format!("`steps` = {}", steps),
        )?;

        let (level, hash) = self.to_nested(cell)?;
        let layer = healpix::nested::get(level);
        let center = layer.center_of_projected_cell(hash);

        let mut out = Vec::with_capacity(count);

        let scale = 1.0 / f64::from(steps - 1);
        for i in 0..steps {
            let u = f64::from(i) * scale;
            for j in 0..steps {
                let v = f64::from(j) * scale;

                // `u` and `v` are generated in `[0, 1]`, so the range check
                // in `spherical_vertex` cannot fail here
                let (lon, lat) = spherical_vertex(center, level, (u, v))?;

                out.push(lon.to_degrees().rem_euclid(360.0));
                out.push(
                    self.ellipsoid
                        .latitude_authalic_to_geographic(lat)
                        .to_degrees(),
                );
            }
        }

        Ok(out)
    }

    pub(crate) fn lonlat_to_healpix_impl(&self, lonlats: &[f64]) -> Result<Vec<u64>, String> {
        if !lonlats.len().is_multiple_of(2) {
            return Err("`lonlats` must be interleaved [lon, lat] pairs (even length)".to_string());
        }

        let mut out = Vec::with_capacity(lonlats.len() / 2);

        // clippy claims that as_chunks is to be preferred over chunks_exact
        for (index, pair) in lonlats.as_chunks::<2>().0.iter().enumerate() {
            // one NaN in a million-element buffer used to abort the call with
            // `RuntimeError: unreachable` — no message and no index
            let hash = self
                .lonlat_to_nested(pair[0], pair[1])
                .map_err(|message| format!("lonlats[{}]: {}", index, message))?;

            out.push(self.nested_to_scheme(hash));
        }

        Ok(out)
    }

    pub(crate) fn healpix_to_lonlat_impl(&self, cells: &[u64]) -> Result<Vec<f64>, String> {
        let mut out = Vec::with_capacity(cells.len() * 2);

        for (index, &cell) in cells.iter().enumerate() {
            let (level, hash) = self
                .to_nested(cell)
                .map_err(|message| format!("cells[{}]: {}", index, message))?;
            let layer = healpix::nested::get(level);
            let (lon, lat) = scalar::healpix_to_lonlat(&hash, layer, &self.ellipsoid);

            out.push(lon);
            out.push(lat);
        }

        Ok(out)
    }

    pub(crate) fn bit_combine_table_impl(&self, size: f64) -> Result<Vec<u64>, String> {
        let size = to_u32(size, "size")?;
        if !size.is_power_of_two() {
            return Err(format!("`size` must be a power of two, got {}", size));
        }
        let nside = self.nside();
        if size > nside {
            return Err(format!(
                "`size` must be at most `nside` ({}) so that every entry is a cell of this grid, got {}",
                nside, size
            ));
        }
        let count = capacity(
            u64::from(size).checked_mul(u64::from(size)),
            &format!("`size` = {}", size),
        )?;

        // the z-order (Morton) interleave depends on the size of the block
        // being unshuffled, not on the level of the grid: deriving it from
        // `self.level` instead silently truncates (`SmallZOC` indexes its
        // lookup table with `i as u8`) and, for level 0, degenerates to
        // cdshealpix's `EMPTY_ZOC`, which returns 0 for every input
        let zoc = healpix::nested::zordercurve::get_zoc(size.trailing_zeros() as u8);

        let mut out = Vec::with_capacity(count);
        for row in 0..size {
            for col in 0..size {
                out.push(self.nested_to_scheme(zoc.ij2h(col, row)));
            }
        }

        Ok(out)
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

    /// All vertices of a `steps` × `steps` subdivision of the given cell, in
    /// one call
    ///
    /// Equivalent to looping `vertex(cell, i / (steps - 1), j / (steps - 1))`
    /// for `i` and `j` in `0..steps` (`i` outer, `j` inner), but the loop
    /// runs inside WASM and the result comes back as a single typed array.
    ///
    /// Returns a `Float64Array` of length `steps * steps * 2`, interleaved as
    /// `[lon0, lat0, lon1, lat1, ...]`. `steps` has to be an integer of at
    /// least 2.
    pub fn vertices(&self, cell: u64, steps: f64) -> Result<Vec<f64>, JsValue> {
        self.vertices_impl(cell, steps)
            .map_err(|message| JsError::new(&message).into())
    }

    /// Center coordinates of the given cells, in one call
    ///
    /// Returns a `Float64Array` of length `cells.length * 2`, interleaved as
    /// `[lon0, lat0, lon1, lat1, ...]`. Rejects the whole batch, naming the
    /// offending index, if any cell id is invalid for this grid.
    ///
    /// For a `zuniq` grid each id is read at the level embedded in it, so
    /// feeding the result back through `lonLatToHealpix` re-encodes every
    /// cell at the grid's level (see `lonLatToHealpix`).
    #[wasm_bindgen(js_name = healpixToLonLat)]
    pub fn healpix_to_lonlat(&self, cells: &[u64]) -> Result<Vec<f64>, JsValue> {
        self.healpix_to_lonlat_impl(cells)
            .map_err(|message| JsError::new(&message).into())
    }

    /// The cells containing the given coordinates, in one call
    ///
    /// `lonlats` is interleaved as `[lon0, lat0, lon1, lat1, ...]`; the
    /// result is a `BigUint64Array` with one cell id per coordinate pair.
    /// Rejects the whole batch, naming the offending index, if any longitude
    /// is not finite or any latitude falls outside `[-90, 90]`.
    ///
    /// For a `zuniq` grid this is **not** the exact inverse of
    /// `healpixToLonLat`: the ids produced here all carry the grid's level,
    /// while `healpixToLonLat` reads each input id at the level embedded in
    /// it. A mixed-level batch therefore comes back entirely at the grid's
    /// level.
    #[wasm_bindgen(js_name = lonLatToHealpix)]
    pub fn lonlat_to_healpix(&self, lonlats: &[f64]) -> Result<Vec<u64>, JsValue> {
        self.lonlat_to_healpix_impl(lonlats)
            .map_err(|message| JsError::new(&message).into())
    }

    /// The full `size` × `size` z-order table, in one call
    ///
    /// Entry `row * size + col` holds `bitCombine(col, row)` — the layout
    /// used when unshuffling a z-order-flattened `size` × `size` chunk into
    /// row-major order.
    ///
    /// `size` is the edge length of the block being unshuffled. The z-order
    /// interleave itself does not depend on the grid's level, but every entry
    /// has to be a cell of this grid, so `size` must be an integer power of
    /// two and at most `nside`.
    #[wasm_bindgen(js_name = bitCombineTable)]
    pub fn bit_combine_table(&self, size: f64) -> Result<Vec<u64>, JsValue> {
        self.bit_combine_table_impl(size)
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

    fn wgs84() -> RustEllipsoid {
        crate::ellipsoid::EllipsoidLike::EllipsoidInverseFlattening(
            crate::ellipsoid::EllipsoidInverseFlattening {
                semi_major_axis: 6378137.0,
                inverse_flattening: 298.257223563,
            },
        )
        .into_ellipsoid()
        .unwrap()
    }

    #[test]
    fn test_vertices_matches_scalar_vertex_over_the_full_gridlook_geometry() {
        // the shape gridlook's `makeHealpixGeometry` builds: 12 base cells ×
        // 65 × 65 vertices, in both sphere and ellipsoid mode. This is the
        // bit-for-bit cross-check the benchmark write-up refers to.
        let steps = 65u32;
        let scale = 1.0 / f64::from(steps - 1);

        for ellipsoid in [RustEllipsoid::default(), wgs84()] {
            let mut geometry = grid(Scheme::Nested, 0);
            geometry.ellipsoid = ellipsoid;

            for cell in 0..12u64 {
                let bulk = geometry.vertices_impl(cell, f64::from(steps)).unwrap();
                assert_eq!(bulk.len(), (steps * steps * 2) as usize);

                for i in 0..steps {
                    for j in 0..steps {
                        let scalar = geometry
                            .vertex_impl(cell, f64::from(i) * scale, f64::from(j) * scale)
                            .unwrap();
                        let offset = ((i * steps + j) * 2) as usize;
                        assert_eq!(bulk[offset], scalar.lon);
                        assert_eq!(bulk[offset + 1], scalar.lat);
                    }
                }
            }
        }
    }

    #[test]
    fn test_vertices_rejects_degenerate_steps() {
        let grid = grid(Scheme::Nested, 4);
        assert!(grid.vertices_impl(164, 1.0).is_err());
        assert!(grid.vertices_impl(164, 0.0).is_err());
    }

    #[test]
    fn test_bulk_methods_reject_invalid_cells() {
        let nested_grid = grid(Scheme::Nested, 4);
        assert!(nested_grid.vertices_impl(3072, 2.0).is_err());

        // the offending index is named rather than trapping the instance on
        // one bad element of a long batch
        let message = nested_grid
            .healpix_to_lonlat_impl(&[0, 164, 3072])
            .unwrap_err();
        assert!(message.starts_with("cells[2]:"), "{}", message);

        let zuniq_grid = grid(Scheme::Zuniq, 4);
        assert!(zuniq_grid.healpix_to_lonlat_impl(&[0]).is_err());
        assert!(zuniq_grid.healpix_to_lonlat_impl(&[u64::MAX]).is_err());
    }

    #[test]
    fn test_lonlat_roundtrip() {
        let grid = grid(Scheme::Ring, 4);
        let cells: Vec<u64> = vec![0, 164, 700];

        let centers = grid.healpix_to_lonlat_impl(&cells).unwrap();
        assert_eq!(centers.len(), cells.len() * 2);

        let roundtrip = grid.lonlat_to_healpix_impl(&centers).unwrap();
        assert_eq!(roundtrip, cells);
    }

    #[test]
    fn test_lonlat_to_healpix_rejects_odd_length() {
        let grid = grid(Scheme::Nested, 4);
        assert!(grid.lonlat_to_healpix_impl(&[45.0, 0.0, 90.0]).is_err());
    }

    #[test]
    fn test_lonlat_to_healpix_rejects_out_of_range_coordinates() {
        // every one of these used to trap the wasm instance inside
        // `Layer::hash`'s `-FRAC_PI_2 <= lat <= FRAC_PI_2` assertion
        for scheme in [Scheme::Nested, Scheme::Ring, Scheme::Zuniq] {
            let grid = grid(scheme, 4);

            assert!(grid.lonlat_to_healpix_impl(&[0.0, 100.0]).is_err());
            assert!(grid.lonlat_to_healpix_impl(&[0.0, -90.000000001]).is_err());
            assert!(grid.lonlat_to_healpix_impl(&[0.0, f64::NAN]).is_err());
            assert!(grid.lonlat_to_healpix_impl(&[f64::NAN, 0.0]).is_err());
            assert!(grid.lonlat_to_healpix_impl(&[f64::INFINITY, 0.0]).is_err());

            // the poles themselves are legal, and so is an unwrapped longitude
            assert!(grid.lonlat_to_healpix_impl(&[0.0, 90.0]).is_ok());
            assert!(grid.lonlat_to_healpix_impl(&[0.0, -90.0]).is_ok());
            assert!(grid.lonlat_to_healpix_impl(&[-720.5, 45.0]).is_ok());
        }
    }

    #[test]
    fn test_lonlat_to_healpix_names_the_offending_index() {
        // one NaN out of a data buffer used to trap the whole instance with
        // `RuntimeError: unreachable`, naming nothing
        let grid = grid(Scheme::Nested, 4);

        let message = grid
            .lonlat_to_healpix_impl(&[45.0, 0.0, 45.0, 0.0, 0.0, 100.0])
            .unwrap_err();
        assert!(message.starts_with("lonlats[2]:"), "{}", message);

        let message = grid.lonlat_to_healpix_impl(&[45.0, f64::NAN]).unwrap_err();
        assert!(message.starts_with("lonlats[0]:"), "{}", message);
        assert!(grid.lonlat_to_healpix_impl(&[f64::NAN, 0.0]).is_err());

        // and the legal batch still works
        assert_eq!(grid.lonlat_to_healpix_impl(&[45.0, 0.0]).unwrap().len(), 1);
    }

    #[test]
    fn test_zuniq_lonlat_methods_are_not_inverses_across_levels() {
        // `healpixToLonLat` reads each id at its embedded level,
        // `lonLatToHealpix` always encodes at the grid's level: a coarse id
        // does not survive the roundtrip. This is the documented rule, pinned
        // here.
        let grid = grid(Scheme::Zuniq, 4);
        let coarse = healpix::nested::to_zuniq(0, 5);
        let fine = healpix::nested::to_zuniq(4, 164);

        let centers = grid.healpix_to_lonlat_impl(&[coarse, fine]).unwrap();
        let roundtrip = grid.lonlat_to_healpix_impl(&centers).unwrap();

        assert_eq!(roundtrip[1], fine);
        assert_ne!(roundtrip[0], coarse);
        // it comes back as the level-4 cell containing the level-0 center
        assert_eq!(healpix::nested::from_zuniq(roundtrip[0]).0, 4);
    }

    #[test]
    fn test_bit_combine_table_matches_scalar() {
        // `bit_combine_table_impl` derives its z-order curve from `size` and
        // `bit_combine_impl` from `self.level`, so the documented equivalence
        // is a claim about two different `zoc`s. `size == nside` is the case
        // where they coincide trivially; `size < nside` is the one that
        // actually tests it.
        for (scheme, level, size) in [
            (Scheme::Nested, 3u8, 8u32),
            (Scheme::Nested, 12, 8),
            (Scheme::Ring, 5, 4),
            (Scheme::Zuniq, 5, 4),
        ] {
            let grid = grid(scheme, level);

            let table = grid.bit_combine_table_impl(f64::from(size)).unwrap();
            assert_eq!(table.len(), (size * size) as usize);
            for row in 0..size {
                for col in 0..size {
                    assert_eq!(
                        table[(row * size + col) as usize],
                        grid.bit_combine_impl(f64::from(col), f64::from(row))
                            .unwrap(),
                        "{:?} level {} size {} at ({}, {})",
                        scheme,
                        level,
                        size,
                        col,
                        row
                    );
                }
            }
        }
    }

    #[test]
    fn test_bulk_methods_reject_non_integer_sizes() {
        // wasm-bindgen's ToUint32 coercion silently turned `2^32 + 2` into 2
        let grid = grid(Scheme::Nested, 4);

        assert!(grid.vertices_impl(164, 4294967298.0).is_err());
        assert!(grid.vertices_impl(164, 2.5).is_err());
        assert!(grid.vertices_impl(164, f64::NAN).is_err());
        assert!(grid.bit_combine_table_impl(4294967298.0).is_err());
        assert!(grid.bit_combine_table_impl(2.5).is_err());
        assert!(grid.bit_combine_table_impl(-4.0).is_err());
    }

    #[test]
    fn test_bit_combine_table_is_level_independent() {
        // the table describes the z-order layout of a `size` × `size` block,
        // which is the same interleave at every level that can hold it
        let size = 8u32;
        let shallow = grid(Scheme::Nested, 3)
            .bit_combine_table_impl(f64::from(size))
            .unwrap();

        for level in 4..=12u8 {
            let deep = grid(Scheme::Nested, level)
                .bit_combine_table_impl(f64::from(size))
                .unwrap();
            assert_eq!(deep, shallow, "level {}", level);
        }
    }

    #[test]
    fn test_bit_combine_table_rejects_size_level_mismatches() {
        // each of these silently returned wrong values (or trapped) when the
        // z-order curve was taken from the grid's level and `size` was
        // unvalidated

        // level 0 gave cdshealpix's EMPTY_ZOC -> every entry 0
        assert!(grid(Scheme::Nested, 0).bit_combine_table_impl(4.0).is_err());
        assert!(grid(Scheme::Zuniq, 0).bit_combine_table_impl(4.0).is_err());
        // size > 256 aliased mod 256 through `SmallZOC`'s `i as u8`
        assert!(
            grid(Scheme::Nested, 8)
                .bit_combine_table_impl(300.0)
                .is_err()
        );
        assert!(
            grid(Scheme::Nested, 8)
                .bit_combine_table_impl(512.0)
                .is_err()
        );
        // size > nside spilled out of the base cell's cell-id range
        assert!(
            grid(Scheme::Nested, 3)
                .bit_combine_table_impl(16.0)
                .is_err()
        );
        // ... which then panicked inside `to_ring`
        assert!(grid(Scheme::Ring, 1).bit_combine_table_impl(256.0).is_err());
        // not a power of two
        assert!(grid(Scheme::Nested, 8).bit_combine_table_impl(3.0).is_err());
        assert!(grid(Scheme::Nested, 8).bit_combine_table_impl(0.0).is_err());

        // the legal cases stay legal, including size == 1 and size == nside
        assert_eq!(
            grid(Scheme::Nested, 0).bit_combine_table_impl(1.0).unwrap(),
            vec![0]
        );
        assert_eq!(
            grid(Scheme::Nested, 8)
                .bit_combine_table_impl(256.0)
                .unwrap()
                .len(),
            65536
        );
    }
}
