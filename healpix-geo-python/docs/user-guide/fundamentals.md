# HEALPix fundamentals and indexing schemes

## Overview

HEALPix (Hierarchical Equal-Area isoLatitude Pixelisation) was originally defined for use in astronomy by [Gorski et al. (2005)](#gorski-2005). It has several important properties:

- **equal area**: all cells of the same refinement level have exactly the same area
- **iso-latitude**: all cells of the same refinement level are arranged around rings of the same latitude
- **hierarchical**: cells are created by recursively and evenly subdividing, which forms a hierarchy of cells

```{figure} healpix_global_indexing.png
:alt: HEALPix nested indexing scheme
:width: 900px
:align: center

The `nested` indexing scheme of HEALPix
```

The lowest refinement level (0) has 12 'base' cells. To go to a higher resolution, each cell is subdivided into 4 'children' cells (they are 'siblings', the original cell is the 'parent').
This makes up- and down-sampling operations trivial.
This means that for a given level, there are $12\cdot4^{\mathrm{level}}$ cells.
The refinement level can go up to 29, where cells have a side-length of approximately 1cm.

:::{note}
To keep compatibility with some other HEALPix libraries, some functions of this package may refer to the refinement level as "depth".
:::

:::{dropdown} Cell statistics table
:name: cell-statistics
:color: primary

```{include} healpix_levels_table.md

```

:::

(indexing-schemes)=

## Indexing schemes

At a given refinement level, each cell has a unique identifier, or 'index'. There are multiple _indexing schemes_ that index and thus sort cells differently:

- `nested`, where indices of sibling cells are close to each other.
- `ring`, where indices of cells on the same iso-latitude rings are close to each other.

Based on `nested`, there are two more schemes that also encode the refinement level in the cell id, which is useful for MOC (Multi-Order Coverage):

- `zuniq`, which represents all cells in the hierarchy depth-first, i.e. descendant cells of a given cell are close numerically.
- `nuniq`, which represents all cells in the hierarchy breadth-first, i.e. cells of the same refinement level are close numerically. This is not currently supported by `healpix-geo`.

### Z-ordering

The `nested` indexing scheme is following a Z-order (aka Morton order).
Note that by bit-shifting by 2 bits to the right, we obtain the parent cell index (at a lower refinement level by one). For instance, sibling cells {4, 5, 6, 7} have a parent cell of index 1.

```{figure} zorder.png
:alt: Example of Z-ordering in nested indexing.
:width: 67%
:align: center
:name: healpix-zorder

Example of Z-ordering in `nested` indexing.
```

### Choosing a scheme

Choosing the appropriate indexing scheme can be important as cells that have neighbouring indices will also be close in memory (when the data is sorted).
For most usages, `nested` is the recommended scheme, though others can be appropriate in some situations:

| Need                    | Recommended Scheme |
| ----------------------- | ------------------ |
| General Application     | **nested**         |
| Hierarchical Navigation | **nested**         |
| Spherical harmonics     | **ring**           |
| Order by latitude       | **ring**           |
| MOC                     | **zuniq**          |

`healpix-geo` provides a module for each indexing scheme
({py:mod}`~healpix_geo.nested`, {py:mod}`~healpix_geo.ring`,
{py:mod}`~healpix_geo.zuniq`) however it is recommended to use functions from the
{py:mod}`~healpix_geo.auto` module which will automatically dispatch to the
correct module. The indexing scheme, refinement level and ellipsoidal model is passed through a {py:class}`~healpix_geo.auto.Grid` object:

```python
import healpix_geo.auto

grid = healpix_geo.auto.Grid(level=8, indexing_scheme="nested", ellipsoid="WGS84")
lon, lat = healpix_geo.auto.healpix_to_lonlat(cell_ids, grid)
```

Each module provides functions to convert indices **from** the module indexing scheme to another one. For instance to change from `ring` to `nested`:

```python
import healpix_geo.ring

cell_ids_nested = healpix_geo.ring.to_nested(cell_ids_ring, depth=8)
```

Or you can use {py:func}`healpix_geo.auto.convert` to convert between any schemes.

:::{important}
Those conversion functions will change the values of indices, but your data will stay in the same order. If necessary, remember to sort your data by indices.
:::

## References

(gorski-2005)=

- Górski et al. (2005), _"HEALPix: A Framework for High-Resolution Discretization and Fast Analysis of Data Distributed on the Sphere."_, The Astrophysical Journal **622**: 759–71. <doi:10.1086/427976>
