# javascript and typescript bindings for `healpix-geo`

This module provides javascript / typescript bindings to the `healpix_geo_core::scalar` crate.

## `Grid`

`Grid` is the recommended entry point: it holds the scheme, the refinement
level and the parsed ellipsoid state (including the authalic-latitude
coefficients), so the per-call overhead is the coordinate math alone, and
every scheme gets the same call shape.

```typescript
import init, { Grid } from "healpix-geo";

await init(); // or use a bundler

const grid = new Grid({
  scheme: "nested", // or "ring" or "zuniq"
  level: 4, // 0-indexed; level 0 is the 12 base cells
  ellipsoid: { semi_major_axis: 6378137.0, inverse_flattening: 298.257223563 },
}); // `ellipsoid` is optional; omit it for the default sphere

const corner = grid.vertex(164n, 0, 0); // u, v are cell-local offsets in [0, 1]
const cell = grid.bitCombine(3, 5);
const ring = grid.toScheme(cell, "ring");
const coarse = grid.toScheme(5n, "zuniq", 0); // read (and encode) 5n at level 0
```

Every method reports misuse — a cell id out of range at the grid's level, a
vertex offset outside `[0, 1]`, a non-integer z-order coordinate — as a
catchable JS `Error` rather than trapping the wasm instance.

Three things worth knowing:

- For the `zuniq` scheme, methods that read a cell id use the level **embedded
  in the id**, not the grid's level; methods that produce one encode the
  grid's level. So `zuniq` → `zuniq` is the identity, and a coarse id survives
  a read unchanged.
- `toScheme` takes an optional `level` that overrides the level the cell is
  read and encoded at. It is only valid when converting to a scheme that
  encodes the level in its cell ids (`"zuniq"` today) and from one that does
  not; anything else throws.
- `grid.ellipsoid` allocates a **new** handle on every read (wasm allocation +
  JS wrapper + finalizer). Hoist it out of hot paths and `free()` it, or use
  the `semiMajorAxis` / `flattening` / `isSphere` getters, which return plain
  numbers.

## Low-level scheme functions

The `nested` / `ring` / `zuniq` namespaces expose the per-scheme functions
`Grid` is built on (their `depth` parameter is the same 0-indexed quantity as
the grid's `level`). Reach for them when a single call is all you need;
otherwise prefer `Grid`, which parses the ellipsoid once instead of per call.

```typescript
import init, * as healpixGeo from "healpix-geo";

await init(); // or use a bundler

// parse once, reuse for any number of calls
const ellipsoid = healpixGeo.Ellipsoid.from({
  semi_major_axis: 6378137.0,
  inverse_flattening: 298.257223563,
}); // or Ellipsoid.from(null) for the default sphere

const cellId: bigint = 10n;
const level: number = 2;
const { lon, lat } = healpixGeo.nested.healpixToLonLat(
  cellId,
  level,
  ellipsoid,
);
```
