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

```{code-cell} python
---
tags: [remove-input]
---
import healpix_geo
```

# Hierarchical Indexing and Neighbours

```{figure} hierarchy_neighbours.svg
:width: 400px
:alt: Scheme showing parent, siblings and neighbours.

Sibling, parent, and neighbours cells.
```

## Neighbours

First, let us recall that on the HEALPix grid there are 8 points on the globe where three cells meet at the same vertex (instead of the usual four).
This means that at each of these points there are three cells that only have 7 immediate neighbours (instead of 8).

:::{important}
The functions presented here will always return arrays of the same size.
If a cell has less neighbours than the maximum possible, the result will be padded with `-1`.

This means that, even though the functions are returning cell indices, the result will be a {py:attr}`numpy.int64` signed integer array instead of the usual unsigned {py:attr}`numpy.uint64`.
:::

```{figure} neighbours_corner.png
:width: 400px

Example of a "corner".

The highlighted cell has only 7 immediate neighbours.
```

Functions retrieving neighbouring cells take in a `ring` parameter that specifies the target degree of connectivity:

- `ring=1` for the 8 immediate neighbours
- `ring=2` for the 16 neighbours of the immediate neighbours
- ...

They are two families of functions available.
{py:func}`~healpix_geo.auto.kth_neighbours` will return the cells in the specified ring, _ie_ in the border of the neighborhood. This means 8 cells for `ring=1`, 16 cells for `ring=2`, ...

{py:func}`~healpix_geo.auto.kth_neighbourhood` will return all the cells inside the neighborhood, including the input, center cell. This means 9 cells for `ring=1`, 25 cells for `ring=2`, ...

```{image} neighbours_direction.png
:width: 300px
:align: right
```

Finally, for a finer control on what neighbouring cells to retrieve, {py:func}`healpix_geo.nested.neighbours` can return in a fixed order either:

- the 4 cells that share an edge (SW, NW, NE, SE),
- the 4 cells that share a vertex, but not an edge (S, W, N, E)
- the 8 immediate neighbours (clockwise starting from S).

## Hierarchical Navigation

:::{note}
This section is only relevant for the `nested` indexing scheme.
:::

To raise the refinement level by one, each cell is subdivided in 4. The original cell is the **parent** and the 4 children are **siblings** of each other.

To obtain the siblings of given cells, one can use {py:func}`healpix_geo.nested.siblings`.
For instance:

```{code-cell} python
healpix_geo.nested.siblings(0, depth=1)
```

The {py:func}`healpix_geo.nested.zoom_to` function allows to move up and down the cell hierarchy. It takes the current level and a target level.
When targeting a higher level (target level > current level), the function will return all descendants of each input cell. For instance:

```{code-cell} python
healpix_geo.nested.zoom_to(0, depth=0, new_depth=2)
```

When targeting a lower level (target level < current level), the function will return the parent of each input cell. For instance:

```{code-cell} python
healpix_geo.nested.zoom_to(12, depth=2, new_depth=0)
```

## See Also

For more details, see:

- {doc}`/tutorials/hierarchical_indexing` - Tutorial on hierarchical indexing and neighbours
