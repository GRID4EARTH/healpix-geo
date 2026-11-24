---
jupytext:
  formats: md:myst
  text_representation:
    extension: .md
    format_name: myst
    format_version: 0.13
    jupytext_version: 1.19.6
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

# Hierarchical indexing and neighbours

## Neighbours

There are several functions to retrieve the neighbours of a given cell.

There are 8 points on the globe where three cells meet at the same vertex, instead of the usual four. Cells in this area might have less neighbours. The functions presented here always return arrays of the same size, but may pad their result with `-1`. See the dedicated [user guide](/user-guide/hierarchical_indexing) for details.

Some functions take in a `ring` parameter that specifies the target degree of connectivity:

- `ring=1` for the 8 immediate neighbours
- `ring=2` for the 16 neighbours of the immediate neighbours
- ...

### Neighbours ring

{py:func}`~healpix_geo.auto.kth_neighbours` returns the cells in the specified ring, _ie_ the border of the neighbourhood.

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
# ring=1 -> 8 immediate neighbours

import numpy as np
from healpix_geo.auto import Grid, kth_neighbours

ipix = 100
grid = Grid(level=8, indexing_scheme="nested")
ring = 1

neighbours = kth_neighbours(ipix, grid, ring=ring)
print(f"{neighbours.size} neighbours")
neighbours
```

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
tags: [hide-input]
---
import matplotlib.pyplot as plt
from matplotlib.collections import PolyCollection
from healpix_geo.auto import healpix_to_lonlat, kth_neighbourhood, vertices

fig, ax = plt.subplots(figsize=(4, 3), dpi=150)

background_ids = kth_neighbourhood(ipix, grid, ring=3).ravel()
background_vertices = np.stack(vertices(background_ids, grid), axis=-1)
ax.add_artist(PolyCollection(background_vertices, ec="k", fc="none"))

neigh_vertices = np.stack(vertices(neighbours[0], grid), axis=-1)
ax.add_artist(PolyCollection(neigh_vertices, ec="k", fc="lightcoral"))

for cell_id in np.concatenate([[ipix], neighbours[0]]):
    lon, lat = healpix_to_lonlat(cell_id, grid)
    ax.annotate(
        cell_id,
        (lon[0], lat[0]),
        ha="center",
        va="center_baseline",
    )

ax.set_xlim(45.5, 46.6)
ax.set_ylim(1.75, 2.7)
ax.set_aspect("equal")
```

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
# ring=2 -> 16 neighbours of immediate neighbours

import numpy as np
from healpix_geo.auto import Grid, kth_neighbours

ipix = 100
grid = Grid(level=8, indexing_scheme="nested")
ring = 2

neighbours = kth_neighbours(ipix, grid, ring=ring)
print(f"{neighbours.size} neighbours")
neighbours
```

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
tags: [hide-input]
---
import matplotlib.pyplot as plt
from matplotlib.collections import PolyCollection
from healpix_geo.auto import healpix_to_lonlat, kth_neighbourhood, vertices

fig, ax = plt.subplots(figsize=(4, 3), dpi=150)

background_ids = kth_neighbourhood(ipix, grid, ring=5).ravel()
background_vertices = np.stack(vertices(background_ids, grid), axis=-1)
ax.add_artist(PolyCollection(background_vertices, ec="k", fc="none"))

neigh_vertices = np.stack(vertices(neighbours[0], grid), axis=-1)
ax.add_artist(PolyCollection(neigh_vertices, ec="k", fc="lightcoral"))

for cell_id in np.concatenate([[ipix], neighbours[0]]):
    lon, lat = healpix_to_lonlat(cell_id, grid)
    ax.annotate(
        cell_id,
        (lon[0], lat[0]),
        ha="center",
        va="center_baseline",
    )

ax.set_xlim(45.15, 46.95)
ax.set_ylim(1.45, 3.05)
ax.set_aspect("equal")
```

+++ {"editable": true, "deletable": true, "slideshow": {"slide_type": ""}}

### Neighbourhood

{py:func}`~healpix_geo.auto.kth_neighbourhood` returns the cells in the specified neighbourhood, including the center cell.

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
# ring=1 -> 9 cells

import numpy as np
from healpix_geo.auto import Grid, kth_neighbourhood

ipix = 100
grid = Grid(level=8, indexing_scheme="nested")
ring = 1

neighbours = kth_neighbourhood(ipix, grid, ring=ring)
print(f"{neighbours.size} neighbours")
neighbours
```

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
tags: [hide-input]
---
import matplotlib.pyplot as plt
from matplotlib.collections import PolyCollection
from healpix_geo.auto import healpix_to_lonlat, kth_neighbourhood, vertices

fig, ax = plt.subplots(figsize=(4, 3), dpi=150)

background_ids = kth_neighbourhood(ipix, grid, ring=3).ravel()
background_vertices = np.stack(vertices(background_ids, grid), axis=-1)
ax.add_artist(PolyCollection(background_vertices, ec="k", fc="none"))

neigh_vertices = np.stack(vertices(neighbours[0], grid), axis=-1)
ax.add_artist(PolyCollection(neigh_vertices, ec="k", fc="lightcoral"))

for cell_id in np.concatenate([[ipix], neighbours[0]]):
    lon, lat = healpix_to_lonlat(cell_id, grid)
    ax.annotate(
        cell_id,
        (lon[0], lat[0]),
        ha="center",
        va="center_baseline",
    )

ax.set_xlim(45.5, 46.6)
ax.set_ylim(1.75, 2.7)
ax.set_aspect("equal")
```

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
# ring=1 -> 25 cells

import numpy as np
from healpix_geo.auto import Grid, kth_neighbourhood

ipix = 100
grid = Grid(level=8, indexing_scheme="nested")
ring = 2

neighbours = kth_neighbourhood(ipix, grid, ring=ring)
print(f"{neighbours.size} neighbours")
neighbours
```

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
tags: [hide-input]
---
import matplotlib.pyplot as plt
from matplotlib.collections import PolyCollection
from healpix_geo.auto import healpix_to_lonlat, kth_neighbourhood, vertices

fig, ax = plt.subplots(figsize=(4, 3), dpi=150)

background_ids = kth_neighbourhood(ipix, grid, ring=5).ravel()
background_vertices = np.stack(vertices(background_ids, grid), axis=-1)
ax.add_artist(PolyCollection(background_vertices, ec="k", fc="none"))

neigh_vertices = np.stack(vertices(neighbours[0], grid), axis=-1)
ax.add_artist(PolyCollection(neigh_vertices, ec="k", fc="lightcoral"))

for cell_id in np.concatenate([[ipix], neighbours[0]]):
    lon, lat = healpix_to_lonlat(cell_id, grid)
    ax.annotate(
        cell_id,
        (lon[0], lat[0]),
        ha="center",
        va="center_baseline",
    )

ax.set_xlim(45.15, 46.95)
ax.set_ylim(1.45, 3.05)
ax.set_aspect("equal")
```

+++ {"editable": true, "deletable": true, "slideshow": {"slide_type": ""}}

### Direction-preserving neighbours

{py:func}`~healpix_geo.nested.neighbours` provides a finer control on what immediate neighbours to return. It can return (in fixed order):

- the 4 cells that share an edge (SW, NW, NE, SE),
- the 4 cells that share a vertex, but not an edge (S, W, N, E)
- the 8 immediate neighbours (clockwise starting from S).

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
import numpy as np
from healpix_geo.nested import neighbours

ipix = 100
level = 8

edge_neighbours = neighbours(ipix, level, connectivity="edge")[0]
vertex_neighbours = neighbours(ipix, level, connectivity="vertex")[0]
all_neighbours = neighbours(ipix, level, connectivity="all")[0]

print(f"Edge:\t{edge_neighbours} (SW, NW, NE, SE)")
print(f"Vertex:\t{vertex_neighbours} (S, W, N, E)")
print(f"All:\t{all_neighbours} (clockwise starting from S)")
```

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
tags: [hide-input]
---
import matplotlib.pyplot as plt
from matplotlib.collections import PolyCollection
from healpix_geo.auto import healpix_to_lonlat, kth_neighbourhood, vertices

fig, ax = plt.subplots(figsize=(4, 3), dpi=150)

edge_vertices = np.stack(vertices(edge_neighbours, grid), axis=-1)
ax.add_artist(PolyCollection(edge_vertices, ec="k", fc="mediumseagreen"))

vertex_vertices = np.stack(vertices(vertex_neighbours, grid), axis=-1)
ax.add_artist(PolyCollection(vertex_vertices, ec="k", fc="plum"))

directions = ["", "S", "SW", "W", "NW", "N", "NE", "E", "SE"]
for direction, cell_id in zip(directions, np.concatenate([[ipix], all_neighbours])):
    lon, lat = healpix_to_lonlat(cell_id, grid)
    ax.annotate(
        cell_id,
        xy=(lon[0], lat[0]),
        xytext=(0, 5),
        textcoords="offset points",
        ha="center",
        va="bottom",
        fontsize=7,
    )
    ax.annotate(
        direction,
        xy=(lon[0], lat[0]),
        ha="center",
        va="center_baseline",
    )

ax.set_xlim(45.5, 46.6)
ax.set_ylim(1.75, 2.7)
ax.set_aspect("equal")
```

+++ {"editable": true, "deletable": true, "slideshow": {"slide_type": ""}}

## Hierarchical indexing

When refining the grid, each **parent** cell is subdivided in 4 **children** cells which are **siblings** of each other.

### Siblings

{py:func}`healpix_geo.nested.siblings` returns the 4 siblings for any input cell (including itself).

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
import healpix_geo.nested

ipix = 1
level = 4

siblings = healpix_geo.nested.siblings(ipix, level)
siblings
```

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
tags: [hide-input]
---
import matplotlib.pyplot as plt
from matplotlib.collections import PolyCollection
from healpix_geo.nested import healpix_to_lonlat, vertices

fig, ax = plt.subplots(figsize=(4, 3), dpi=150)

background_vertices = np.stack(vertices(np.arange(16), level), axis=-1)
ax.add_artist(PolyCollection(background_vertices, ec="k", fc="none"))

siblings_vertices = np.stack(vertices(siblings[0], level), axis=-1)
ax.add_artist(PolyCollection(siblings_vertices, ec="k", fc="mediumseagreen"))

for cell_id in np.arange(16):
    lon, lat = healpix_to_lonlat(cell_id, level)
    ax.annotate(
        cell_id,
        xy=(lon[0], lat[0]),
        ha="center",
        va="center_baseline"
    )

ax.set_xlim(33, 57)
ax.set_ylim(-0.5, 20)
ax.set_aspect("equal")
```

+++ {"editable": true, "deletable": true, "slideshow": {"slide_type": ""}}

### Zoom

The {py:func}`healpix_geo.nested.zoom_to` function allows to move up and down the cell hierarchy. It takes the current level and a target level.

When targeting a higher level (target level > current level), the function will give all descendants of each input cell.

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
# we keep the same example as above
import healpix_geo.nested

healpix_geo.nested.zoom_to(0, depth=2, new_depth=4)
```

+++ {"editable": true, "deletable": true, "slideshow": {"slide_type": ""}}

When targeting a lower level (target level < current level), the function will give the parent of each input cell. For instance:

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
import healpix_geo.nested

healpix_geo.nested.zoom_to(9, depth=4, new_depth=2)
```
