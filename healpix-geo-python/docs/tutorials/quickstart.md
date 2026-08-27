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

# Quickstart

This tutorial presents you the fundamental concepts of `healpix-geo` through practical examples.

## First example : Coordinates conversion

The most common task is to convert geographic coordinates into HEALPix cell identifiers:

```{code-cell} python
import numpy as np
from healpix_geo.nested import lonlat_to_healpix, healpix_to_lonlat

# Define some points of interest (longitude, latitude in degrees)
paris = (2.3522, 48.8566)
tokyo = (139.6917, 35.6895)
new_york = (-74.0060, 40.7128)

lon = np.array([paris[0], tokyo[0], new_york[0]])
lat = np.array([paris[1], tokyo[1], new_york[1]])

# Convert in HEALPix cells level 8
depth = 8
ipix = lonlat_to_healpix(lon, lat, depth, ellipsoid="WGS84")

print(f"HEALPix cells (depth={depth}):")
print(f"  Paris: {ipix[0]}")
print(f"  Tokyo: {ipix[1]}")
print(f"  New York: {ipix[2]}")
```

:::{note}
Cell indices are unique integers that identify each cell. Each city is in a different cell at level 8.
:::

## Understand `depth` parameter

The `depth` parameter (or level) controls the tiling resolution :

- `depth=0` : 12 cells around the world (~6 520 km per cell)
- `depth=8` : 786 432 cells (~25 km per cell)
- `depth=12` : 201 326 592 cells (~1.6 km per cell)

:::{seealso}
See {doc}`../healpix/levels` for a complete table of levels with their resolutions.
:::

### Example: Same point at different resolutions

```{code-cell} python
# Same point at different resolutions
lon, lat = np.array([2.3522]), np.array([48.8566])  # Paris

for depth in [0, 4, 8, 12]:
    ipix = lonlat_to_healpix(lon, lat, depth, ellipsoid="WGS84")
    print(f"Depth {depth:2d}: cell {ipix[0]:10d}")
```

:::{tip}
**General rule**: Increasing the `depth` by 1 multiplies the number of cells by 4 and divides the cell size by 2.
:::

## Reverse conversion: cell → coordinates

Recover the coordinates of the **center** of a cell:

```{code-cell} python
# Obtain coordinates of the cell center
ipix = np.array([349440])  # Paris
depth = 8

lon_center, lat_center = healpix_to_lonlat(ipix, depth, ellipsoid="WGS84")
print(f"Cell center {ipix[0]}:")
print(f"  Longitude: {lon_center[0]:.4f}°")
print(f"  Latitude:  {lat_center[0]:.4f}°")
```

:::{note}
The center coordinates may be slightly different from the original coordinates, because we have "rounded" to the nearest cell.
:::

## Research of neighbour

HEALPix makes it easy to find **neighboring** cells:

```{code-cell} python
from healpix_geo.nested import kth_neighbourhood

ipix = np.array([100])
depth = 5
k = 1  # direct neighbour (distance 1)

neighbours = kth_neighbourhood(ipix, depth, k)
print(f"Direct neighbourhoods of the cell {ipix[0]}:")
print(neighbours)
```

:::{seealso}
For more details on hierarchy and neighbourhood, see {doc}`../user-guide/hierarchical_indexing`.
:::

## Hierarchic navigation

```{code-cell} python
from healpix_geo.nested import zoom_to

parent_ipix = np.array([100])
parent_depth = 5

# Obtain the 4 children cells
children_ipix = zoom_to(parent_ipix, parent_depth, parent_depth + 1)

print(f"Parent cell:  {parent_ipix[0]} (depth={parent_depth})")
print(f"Children cell: {children_ipix} (depth={parent_depth + 1})")
```

## Sibling

```{code-cell} python
import numpy as np
from healpix_geo.nested import siblings, healpix_to_lonlat

depth = 3

# Choose one cell
ipix = np.array([42], dtype=np.uint64)

# Get siblings (same parent)
sib = siblings(ipix, depth)

print(f"Cell: {ipix[0]} at depth {depth}")
print(f"Siblings: {sib}")
```

Visualisation :

```{code-cell} python
---
tags: [hide-input]
---
import numpy as np
import matplotlib.pyplot as plt
from healpix_geo.nested import siblings, healpix_to_lonlat

depth = 3
ipix = np.array([42], dtype=np.uint64)

# Get siblings
sib = siblings(ipix, depth)

# Get coordinates
lon_sib, lat_sib = healpix_to_lonlat(sib, depth, ellipsoid="sphere")
lon_cell, lat_cell = healpix_to_lonlat(ipix, depth, ellipsoid="sphere")

# Plot
fig, ax = plt.subplots(figsize=(8, 4))

# Plot siblings
ax.scatter(lon_sib, lat_sib, s=200, label="Siblings")

# Highlight original cell
ax.scatter(lon_cell, lat_cell, s=250, marker="*", label="Selected cell")

ax.set_xlim(-180, 180)
ax.set_ylim(-90, 90)
ax.set_xlabel("Longitude")
ax.set_ylabel("Latitude")
ax.set_title(f"Siblings at depth {depth}")
ax.legend()

plt.tight_layout()
plt.show()
```

## Next Steps

Now you know basics, you can explore :

::::{grid} 1 1 2 2
:gutter: 2

:::{grid-item-card} Ellipsoids
:link: ellipsoid_basics
:link-type: doc

:::

:::{grid-item-card} Advanced Conversions
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
