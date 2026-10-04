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

# Cover requests

This tutorial explains how to find all the HEALPix cells which intersect a geographic region.

## Type of requests

### 1. Cone Coverage

Find all cells within a given radius around a point.

```{code-cell} python
import numpy as np
from healpix_geo.nested import cone_coverage

# Center and radius
lon_center = 2.3522  # Paris
lat_center = 48.8566
radius_deg = 1.0  # radius in degrees (~111 km)
depth = 8

cells = cone_coverage((lon_center, lat_center), radius_deg, depth, ellipsoid="WGS84")

print(f"Number of cells in the radius: {len(cells)}")
```

#### Query many centers together

Pass an `(N, 2)` array to the same function to evaluate cones in native
parallel code. The radius is shared by all centers. The three results are
`RaggedArray` objects: rows may have different lengths, and share a single
offsets array without padding. Single-center `(2,)` inputs still return
three ordinary NumPy arrays.

```{code-cell} python
centers = np.array([[2.3522, 48.8566], [179.999, 0.0], [12.0, 89.9]])
cell_ids, depths, covered = cone_coverage(
    centers, 0.5, 8, ellipsoid="WGS84", num_threads=8
)
start, stop = cell_ids.offsets[:2]
paris_cells = cell_ids.data[start:stop]
print(f"Cells intersecting the first cone: {paris_cells.size}")
```

Rows retain exactly the order and values of individual scalar calls.
`num_threads=0` selects available parallelism, capped at eight workers.
Empty `(0, 2)` and single-row `(1, 2)` arrays retain the batched return type.

### 2. Box Coverage

Find all cells in a spherical rectangle.

```{code-cell} python
from healpix_geo.nested import box_coverage

# Coverage box
lon_min, lat_min = 2.0, 48.5
lon_max, lat_max = 3.0, 49.0

center = (
    0.5 * (lon_min + lon_max),
    0.5 * (lat_min + lat_max),
)

size = (
    lon_max - lon_min,
    lat_max - lat_min,
)

angle = 0.0
depth = 8

cells = box_coverage(center, size, angle, depth, ellipsoid="WGS84", flat=True)

print(f"Cells number : {len(cells)}")
```

### 3. Polygon coverage

Find all cells in a polygon coverage.

```{code-cell} python
from healpix_geo.nested import polygon_coverage
import numpy as np

vertices = np.array([[2.0, 48.5], [3.0, 48.5], [2.5, 49.0]])

depth = 8

cells = polygon_coverage(vertices, depth, ellipsoid="WGS84", flat=True)

print(f"Cells in the polygon : {len(cells)}")
```

## Summary

### Principal functions working on geometries

| Function                   | Usage                   | Key parameters                                  |
| -------------------------- | ----------------------- | ----------------------------------------------- |
| `zone_coverage`            | Zone request            | bbox, depth                                     |
| `cone_coverage`            | Circular request        | center, radius, depth                           |
| `box_coverage`             | Rectangular request     | center, size, angle, depth                      |
| `elliptical_cone_coverage` | Elliptical cone request | center, ellipse_geometry, position_angle, depth |
| `polygon_coverage`         | Polygonal request       | vertices, depth                                 |

### Principal functions working on a region represented by cells

| Function            | Usage      | Key parameters |
| ------------------- | ---------- | -------------- |
| `internal_boundary` | Boundaries | depth, ipix    |

## Next Steps

::::{grid} 1 1 2 2
:gutter: 2

:::{grid-item-card} Working MOC
:link: working_with_moc
:link-type: doc
:::

:::{grid-item-card} Performance
:link: performance_optimization
:link-type: doc

:::

:::{grid-item-card} Hierarchy
:link: ../user-guide/hierarchical_indexing
:link-type: doc
:::

:::{grid-item-card} Api reference
:link: ../api
:link-type: doc

:::

::::
