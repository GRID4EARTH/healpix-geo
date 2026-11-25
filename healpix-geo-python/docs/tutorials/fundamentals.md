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

# HEALPix fundamentals and indexing schemes

## Preamble

The HEALPix grid can be defined with different **refinement levels** (or depth). A higher level has smaller cells. The lowest level, 0, has 12 base cells. To obtain a level one higher, each cell is divided in 4 children cells. This means that for a given level, there are $12\cdot 4^{\mathrm{level}}$ cells.
The highest level is 29, with $\sim 3\cdot 10^{18}$ cells of size 1cm.

For a given refinement level, each cell is identified by a unique index. `healpix-geo` supports multiple indexing schemes, _ie_ ways of ordering the cells:

- `ring`, where indices of cells on the same iso-latitude rings are close to each other.
- `nested`, where indices of sibling cells are close to each other.
- `zuniq`, based on nested, which encode the refinement level in the index itself.

For most usages, `nested` is the recommended scheme, though others can be appropriate in some situations:

| Need                    | Recommended Scheme |
| ----------------------- | ------------------ |
| General Application     | **nested**         |
| Hierarchical Navigation | **nested**         |
| Spherical harmonics     | **ring**           |
| Order by latitude       | **ring**           |
| MOC                     | **zuniq**          |

### Auto module

`healpix-geo` provides a separate module for each indexing scheme: {py:mod}`~healpix_geo.nested`, {py:mod}`~healpix_geo.ring`, and {py:mod}`~healpix_geo.zuniq`. They contain the same functions (with some exceptions). The {py:mod}`~healpix_geo.auto` module dispatch computations to the correct indexing scheme according to the `grid` parameter. This is the recommend interface as it avoids hard-coding the indexing scheme in your scripts.

The `grid` parameter is a {py:class}`~.auto.Grid` object containing the indexing scheme, the refinement level (except for `zuniq`) and the ellipsoidal model.

### Input types

All functions are vectorized and accept array-like inputs. For instance for the cell indices, all functions will accept:

- numpy arrays
- scalar integer (automatically converted to a 1D array)
- nested sequences of integers or numpy array

For details, see the [](/api).

## Coordinate to cell index

We can find the cell index of a given coordinate for different refinement levels.

```{code-cell} ipython3
from healpix_geo.auto import Grid, lonlat_to_healpix

lon, lat = 2.3522, 48.8566  # Paris

for level in [0, 4, 8, 12]:
    ipix = lonlat_to_healpix(
        lon, lat, Grid(level=level, indexing_scheme="nested", ellipsoid="WGS84")
    )
    print(f"Level {level:2d}: cell {ipix[0]:10d}")
```

Similarly, we can get the cell index in different schemes for the same location.

```{code-cell} ipython3
from healpix_geo.auto import Grid, lonlat_to_healpix

lon, lat = 2.3522, 48.8566  # Paris

for scheme in ["nested", "ring", "zuniq"]:
    ipix = lonlat_to_healpix(lon, lat, Grid(level=8, indexing_scheme=scheme, ellipsoid="WGS84"))
    print(f"{scheme} scheme: \t cell {ipix[0]}")
```

## Vectorization

All functions are vectorized and will return outputs with the same shape as the input except:

- scalars are automatically converted to a 1D array
- some functions may add an axis to the input, but the rest of the shape is conserved. Check the functions documentation for details.

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
import numpy as np
from healpix_geo.auto import Grid, healpix_to_lonlat

cell_ids = np.arange(3*5).reshape((3, 5))
grid = Grid(level=8, indexing_scheme="nested")

# This returns the coordinates of the cells center
lon, lat = healpix_to_lonlat(cell_ids, grid)

print(f"Input shape: {cell_ids.shape}")
print(f"Output shape: {lon.shape}")
```

## Convert the indexing scheme

The preferred way to convert cell indices to a different scheme is to use {py:func}`healpix_geo.auto.convert`.

Note that each module provides functions to convert indices **from** the module indexing scheme to another one. For instance to change from `nested` to `ring`, the next cells are equivalent:

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
import numpy as np

level = 4
cell_ids_nested = np.arange(12*4**level)
cell_ids_nested
```

```{code-cell} ipython3
import healpix_geo.nested

cell_ids_ring = healpix_geo.nested.to_ring(cell_ids_nested, depth=level)
cell_ids_ring
```

```{code-cell} ipython3
from healpix_geo.auto import Grid, convert

# the current (nested) scheme
grid = Grid(level=4, indexing_scheme="nested")
cell_ids_ring, _ = convert(cell_ids_nested, grid, to="ring")
cell_ids_ring
```

At this point the indices have been converted but the data is still in the same order. Remember to sort by indices, for instance in numpy:

```{code-cell} ipython3
data = np.random.rand(cell_ids_nested.size)

sorting_ind = np.argsort(cell_ids_ring)
cell_ids_ring = cell_ids_ring[sorting_ind]
data = data[sorting_ind]
```
