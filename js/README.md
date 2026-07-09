# javascript and typescript bindings for `healpix-geo`

This module provides javascript / typescript bindings to the `healpix_geo_core::scalar` crate.

Usage:

```typescript
import init, * as healpixGeo from "healpix-geo";

await init(); // or use a bundler

const cellId: bigint = 10;
const level: number = 2;
const { lon, lat } = healpixGeo.nested.healpixToLonLat(
  cellId,
  level,
  ellipsoid,
);
```
