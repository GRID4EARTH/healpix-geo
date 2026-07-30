# javascript and typescript bindings for `healpix-geo`

This module provides javascript / typescript bindings to the `healpix_geo_core::scalar` crate.

Usage:

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
