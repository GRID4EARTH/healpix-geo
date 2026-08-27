---
jupytext:
  formats: md:myst
  text_representation:
    extension: .md
    format_name: myst
kernelspec:
  name: python3
  display_name: Python 3
---

# Ellipsoids basics

This tutorial explain why and how to use ellipsoids of reference in `healpix-geo`.

## Supported Ellipsoids

```{code-cell} python
import numpy as np
from healpix_geo.nested import lonlat_to_healpix

# Test Point
lon, lat = np.array([0.0]), np.array([45.0])
depth = 10

ellipsoids = {
    "sphere": "Unit sphere",
    "WGS84": "Global geodetic standard",
    "GRS80": "Geodetic Reference System 1980",
    "WGS72": "World Geodetic System 1972",
}


print(f"HEALPix cell at point (0°, 45°) at depth={depth}:\n")
for ellipsoid, description in ellipsoids.items():
    ipix = lonlat_to_healpix(lon, lat, depth, ellipsoid=ellipsoid)
    print(f"{ellipsoid:10s}: {ipix[0]:10d}  # {description}")
```

:::{seealso}
For a list of supported named ellipsoids, see [here](https://github.com/busstoptaktik/geodesy/blob/f9090b8c91f401892a93979f100fa4d987eb0836/src/ellipsoid/constants.rs#L6-L54).
:::

Instead of using a predefined ellipsoid name, you can define a custom ellipsoid by explicitly providing its geometric parameters:

```python
ipix = lonlat_to_healpix(lon, lat, depth, ellipsoid={"radius": 6371000.0})
ipix = lonlat_to_healpix(
    lon,
    lat,
    depth,
    ellipsoid={"semimajor_axis": 6378132.0, "inverse_flattening": 300.0},
)
```

The dictionary must contain either:

- `"radius"` for a spherical model, or
- `"semimajor_axis"` and `"inverse_flattening"` for an ellipsoidal model.

### Which ellipsoid choose ?

:::{tip}
For most real-world geospatial applications, use **WGS84**, as it is the current global geodetic standard.
Use a spherical model (**sphere**) only when high accuracy is not required.
:::

## Typical use case

### GPS data

```{code-cell} python
import numpy as np
from healpix_geo.nested import lonlat_to_healpix

# GPS trajectory data
gps_points = np.array(
    [
        [2.3522, 48.8566],  # Paris
        [2.3532, 48.8576],  # 100m further
        [2.3542, 48.8586],  # 100m further
    ]
)

lon_gps = gps_points[:, 0]
lat_gps = gps_points[:, 1]
depth = 29  # Highest resolution

# Always use WGS84 for GPS
ipix = lonlat_to_healpix(lon_gps, lat_gps, depth, ellipsoid="WGS84")
print(f"HEALPix cells of GPS trajectory:")
print(ipix)
```

:::{tip}
For more information see {doc}`../healpix/ellipsoidal-extension`
:::

## Next steps

::::{grid} 1 1 2 2
:gutter: 2

:::{grid-item-card} Reference systems
:link: ../reference-system
:link-type: doc

:::

:::{grid-item-card} Advanced conversion
:link: coordinate_conversion
:link-type: doc
:::

:::{grid-item-card} Cover Requests
:link: coverage_queries
:link-type: doc
:::

:::{grid-item-card} Performance
:link: performance_optimization
:link-type: doc
:::

::::
