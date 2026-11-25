---
jupytext:
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

# Coordinates and cell geometry

This tutorial explores how to retrieve HEALPix cells geometry and how to convert geographical coordinates to cell indices.

## Coordinates systems

`healpix-geo` supports two types of geographical coordinates: longitude/latitude and cartesian.

- For longitude/latitude, corresponding functions will take in or return:
  - `lon`: Longitude in decimal degrees, positive towards the East, 0° at Greenwich median.
    Functions will output the longitude in [0, 360], but will also accept inputs in [-180, 180].
  - `lat`: Latitude in decimal degrees, in [-90, 90], positive towards the North, 0° at the Equator.

- For cartesian coordinates, corresponding functions will take in or return `x`, `y`, and `z` in meters. The x-axis points from the Earth center to the point (lon=0, lat=0), the y-axis points to (lon=90, lat=0) and the z-axis points to the North pole.

:::{note}
All coordinates passed (`lon`/`lat` or `x`/`y`/`z`) must have the same shape, no broadcasting is done.
:::

## Cell geometry

### Centers

{py:func}`~healpix_geo.auto.healpix_to_lonlat` (and {py:func}`~healpix_geo.auto.healpix_to_cartesian`) returns the coordinates of the cell **centers**.

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
from healpix_geo.auto import Grid, healpix_to_lonlat

ipix = 42
grid = Grid(level=8, indexing_scheme="nested", ellipsoid="WGS84")

lon, lat = healpix_to_lonlat(ipix, grid)

print(f"Cell index: {ipix} (level {grid.level})")
print("Cell center")
print(f"  Longitude: {lon[0]:.4f}°")
print(f"  Latitude: {lat[0]:.4f}°")
```

### Vertices

{py:func}`~healpix_geo.auto.vertices` returns the longitude and latitude of the **vertices** of the cell, in the order (S, E, N, W).

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
import numpy as np
import matplotlib.pyplot as plt
from healpix_geo.auto import Grid, healpix_to_lonlat, vertices

ipix = 42
grid = Grid(level=8, indexing_scheme="nested", ellipsoid="WGS84")

lon_vertex, lat_vertex = vertices(ipix, grid)
lon_center, lat_center = healpix_to_lonlat(ipix, grid)

fig, ax = plt.subplots(figsize=(3, 2), dpi=150)
ax.scatter(lon_center, lat_center, marker="x")
ax.add_artist(plt.Polygon(np.stack([lon_vertex[0], lat_vertex[0]], axis=-1), ec="k", fc="none"))
ax.scatter(lon_vertex[0], lat_vertex[0], c="k", s=3)

for i, (lon, lat) in enumerate(zip(lon_vertex[0], lat_vertex[0])):
    ax.annotate(
        i,
        xy=(lon, lat),
        xytext=(0, -3),
        textcoords="offset points",
        ha="center",
        va="top"
    )

ax.set_xlim(43.5, 44.0)
ax.set_ylim(1.0, 1.4)
ax.set_aspect("equal")
```

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.collections import PolyCollection
from healpix_geo.auto import Grid, healpix_to_lonlat, vertices

cell_ids = np.arange(32, 48)
grid = Grid(level=8, indexing_scheme="nested", ellipsoid="WGS84")

lon_vertex, lat_vertex = vertices(cell_ids, grid)
lon_center, lat_center = healpix_to_lonlat(cell_ids, grid)

fig, ax = plt.subplots(figsize=(4, 3), dpi=150)

poly = PolyCollection(np.stack([lon_vertex, lat_vertex], axis=-1), ec="k", fc="none")
ax.add_collection(poly)

ax.scatter(lon_center, lat_center, s=2)
for cell_id, lon, lat in zip(cell_ids, lon_center, lat_center):
    ax.annotate(
    cell_id,
    xy=(lon, lat),
    xytext=(0, -3),
    textcoords="offset points",
    ha="center",
    va="top"
    )

ax.set_aspect("equal")
```

## Obtaining cell indices

{py:func}`~healpix_geo.auto.lonlat_to_healpix` (and {py:func}`~healpix_geo.auto.cartesian_to_healpix`) returns the indices of the cells containing given geographical coordinates.

```{code-cell} ipython3
import numpy as np

from healpix_geo.auto import Grid, lonlat_to_healpix

points = np.array(
    [
        [5.27038, 47.46735],
        [5.11568, 47.01814],
        [5.24220, 47.18502],
    ]
)
lon, lat = points.T

grid = Grid(level=7, indexing_scheme="nested", ellipsoid="WGS84")
cell_ids = lonlat_to_healpix(lon, lat, grid)
cell_ids
```

```{code-cell} ipython3
import matplotlib.pyplot as plt
from matplotlib.collections import PolyCollection
from healpix_geo.auto import vertices


fig, ax = plt.subplots(figsize=(4, 3), dpi=150)

poly = PolyCollection(np.stack(vertices(np.unique(cell_ids), grid), axis=-1), ec="k", fc="none")
ax.add_collection(poly)

ax.scatter(lon, lat, s=2)

ax.set_aspect("equal")
```
