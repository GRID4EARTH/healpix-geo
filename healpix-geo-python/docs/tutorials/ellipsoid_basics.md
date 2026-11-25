---
jupytext:
  formats: md:myst
  text_representation:
    extension: .md
    format_name: myst
    format_version: 0.13
    jupytext_version: 1.19.7
kernelspec:
  name: python3
  display_name: Python 3 (ipykernel)
  language: python
---

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
tags: [remove-cell]
---
%matplotlib inline
```

+++ {"editable": true, "deletable": true, "slideshow": {"slide_type": ""}}

# Ellipsoids basics

`healpix-geo` extends standard HEALPix, defined only on the sphere, to any ellipsoidal model. See the dedicated [user-guide](/user-guide/ellipsoids) for details on how it does it.

## Specifying the ellipsoid

The ellipsoidal model to use can be specified via the `ellipsoid` keyword argument to the {py:class}`.Grid` object if you are using the {py:mod}`.auto` module, or directly in the function otherwise.

The ellipsoidal model can be specified either by name or by parameters. Available names are those built-in in the [geodesy rust crate](https://github.com/busstoptaktik/geodesy/blob/f9090b8c91f401892a93979f100fa4d987eb0836/src/ellipsoid/constants.rs#L6-L54).
Here are some of the available names:

| name   |                                        |
| ------ | -------------------------------------- |
| sphere | Normal sphere (r=6370997), **default** |
| WGS84  | WGS 84 (current standard)              |
| WGS72  | WGS 72 (old standard)                  |
| GRS80  | GRS 80                                 |
| intl   | International 1909 (Hayford)           |

:::{tip}
For most real-world geospatial applications, use **WGS84**, as it is the current global geodetic standard.
Use a spherical model (**sphere**) only when high accuracy is not required.
:::

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
from healpix_geo.auto import Grid, healpix_to_lonlat

ipix = 42
level = 10

ellipsoids = {
    "sphere": "Unit sphere",
    "WGS84": "Global geodetic standard",
    "GRS80": "Geodetic Reference System 1980",
    "WGS72": "World Geodetic System 1972",
}

print(f"Center of cell #{ipix} (level {level}):\n")
for ellipsoid, description in ellipsoids.items():
    lon, lat = healpix_to_lonlat(ipix, grid=Grid(level=level, indexing_scheme="nested", ellipsoid=ellipsoid))
    print(f"{ellipsoid:}:\t{lon[0]:.8f}° ; {lat[0]:.8f}°  # {description}")
```

+++ {"editable": true, "deletable": true, "slideshow": {"slide_type": ""}}

The ellipsoidal model can also be specified by a dictionary of parameters:

- `"radius"` (in meters) for a spherical model,
- `"semimajor_axis"` (in meters) and `"inverse_flattening"` for an ellipsoidal model.

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
import numpy as np
from healpix_geo.auto import Grid, lonlat_to_healpix

ipix = 42
grid=Grid(
    level=10,
    indexing_scheme="nested",
    ellipsoid={"semimajor_axis": 6378132.0, "inverse_flattening": 300.0},
)

lon, lat = healpix_to_lonlat(ipix, grid)
print(f"Cell index: {ipix} (level {grid.level})")
print("Cell center")
print(f"  Longitude: {lon[0]:.4f}°")
print(f"  Latitude: {lat[0]:.4f}°")
```
