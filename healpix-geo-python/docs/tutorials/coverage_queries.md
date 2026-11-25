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

# Coverage queries

`healpix-geo` provides a number of functions to obtain the indices of cells that fall within types of geographic areas.
All these functions return three arrays of:

- the indices of cells that fall at least partially in the target region,
- the refinement level of each cell,
- whether each cell is fully covered by the target region.

| Function                   | Usage                   | Key parameters                           |
| -------------------------- | ----------------------- | ---------------------------------------- |
| `zone_coverage`            | Zone request            | bbox                                     |
| `cone_coverage`            | Circular request        | center, radius                           |
| `box_coverage`             | Rectangular request     | center, size, angle                      |
| `elliptical_cone_coverage` | Elliptical cone request | center, ellipse_geometry, position_angle |
| `polygon_coverage`         | Polygonal request       | vertices                                 |

:::{note}
All regions are defined on the sphere (or ellipsoid), so there can be small discreptancies in the following plots.
:::

## Requests types

### Zone (bounding box)

{py:func}`~healpix_geo.auto.zone_coverage` return cells that fall in a rectangular area defined by its minimum and maximum longitude and latitude.

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
from healpix_geo.auto import Grid, zone_coverage

grid = Grid(level=8, indexing_scheme="nested")
bbox = (10.0, 40.0, 20.0, 47.0)
cell_ids, _, fully_covered = zone_coverage(bbox, grid)
cell_ids
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
import numpy as np
from matplotlib.collections import PolyCollection
from matplotlib.transforms import Bbox
from healpix_geo.auto import vertices

fig, ax = plt.subplots(figsize=(4, 3), dpi=150)

partial_vertices = np.stack(vertices(cell_ids[~fully_covered], grid), axis=-1)
in_bbox_vertices = np.stack(vertices(cell_ids, grid), axis=-1)
ax.add_collection(PolyCollection(partial_vertices, ec="none", fc="lightcoral"))
ax.add_collection(PolyCollection(in_bbox_vertices, ec="0.5", fc="none"))

b = Bbox.from_extents(bbox)
ax.add_artist(plt.Rectangle(b.p0, b.width, b.height, ec="royalblue", fc="none", lw=2))

ax.set_aspect("equal")
```

+++ {"editable": true, "deletable": true, "slideshow": {"slide_type": ""}}

### Rectangle

{py:func}`~healpix_geo.auto.box_coverage` return cells that fall in a rotated rectangular area defined by its center point, size, and rotation angle.

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
from healpix_geo.auto import Grid, box_coverage

grid = Grid(level=8, indexing_scheme="nested")
center = (15.0, 20.0)
height = 5.0
width = 3.0
angle = 30.0

cell_ids, _, fully_covered = box_coverage(center, (height, width), angle, grid=grid)
cell_ids
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
import numpy as np
from matplotlib.collections import PolyCollection
from healpix_geo.auto import vertices

fig, ax = plt.subplots(figsize=(4, 3), dpi=150)

partial_vertices = np.stack(vertices(cell_ids[~fully_covered], grid), axis=-1)
in_bbox_vertices = np.stack(vertices(cell_ids, grid), axis=-1)
ax.add_collection(PolyCollection(partial_vertices, ec="none", fc="lightcoral"))
ax.add_collection(PolyCollection(in_bbox_vertices, ec="0.5", fc="none"))

ax.add_artist(
    plt.Rectangle(
        np.array(center) - np.array([width, height]),
        2*width,
        2*height,
        angle=-angle,
        rotation_point="center",
        ec="royalblue",
        fc="none",
        lw=2
    )
)

ax.set_aspect("equal")
```

+++ {"editable": true, "deletable": true, "slideshow": {"slide_type": ""}}

### Cone

{py:func}`~healpix_geo.auto.cone_coverage` return cells that fall within a cone, _ie_ within a given radius from a center position.
This function relies on approximations: it may return cells that are near the cone but are not covered by it, and some cells might be tagged as partially covered but may be fully covered. However it guarantees that cells marked as fully covered are really fully covered.

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
from healpix_geo.auto import Grid, cone_coverage

grid = Grid(level=8, indexing_scheme="nested")
center = (15.0, 20.0)
radius = 5.0

cell_ids, _, fully_covered = cone_coverage(center, radius, grid=grid)
cell_ids
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
import numpy as np
from matplotlib.collections import PolyCollection
from healpix_geo.auto import vertices

fig, ax = plt.subplots(figsize=(4, 3), dpi=150)

partial_vertices = np.stack(vertices(cell_ids[~fully_covered], grid), axis=-1)
in_bbox_vertices = np.stack(vertices(cell_ids, grid), axis=-1)
ax.add_collection(PolyCollection(partial_vertices, ec="none", fc="lightcoral"))
ax.add_collection(PolyCollection(in_bbox_vertices, ec="0.5", fc="none"))

ax.add_artist(plt.Circle(center, radius, ec="royalblue", fc="none", lw=2))

ax.set_aspect("equal")
```

+++ {"editable": true, "deletable": true, "slideshow": {"slide_type": ""}}

### Elliptical cone

{py:func}`~healpix_geo.auto.elliptical_cone_coverage` return cells that fall within an elliptical cone, defined by its center position, its semi minor and major axes, and orientation angle.
Similarly to the cone, this functions relies on approximations.

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
from healpix_geo.auto import Grid, elliptical_cone_coverage

grid = Grid(level=8, indexing_scheme="nested")
center = (15.0, 20.0)
a = 6
b = 3
angle = 20

cell_ids, _, fully_covered = elliptical_cone_coverage(center, (a, b), angle, grid=grid)
cell_ids
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
import numpy as np
from matplotlib.collections import PolyCollection
from matplotlib.patches import Ellipse
from healpix_geo.auto import vertices

fig, ax = plt.subplots(figsize=(4, 3), dpi=150)

partial_vertices = np.stack(vertices(cell_ids[~fully_covered], grid), axis=-1)
in_bbox_vertices = np.stack(vertices(cell_ids, grid), axis=-1)
ax.add_collection(PolyCollection(partial_vertices, ec="none", fc="lightcoral"))
ax.add_collection(PolyCollection(in_bbox_vertices, ec="0.5", fc="none"))

ax.add_artist(
    Ellipse(
        center,
        2*a,
        2*b,
        angle=90-angle,
        ec="royalblue",
        fc="none",
        lw=2,
    )
)

ax.set_aspect("equal")
```

+++ {"editable": true, "deletable": true, "slideshow": {"slide_type": ""}}

### Polygon coverage

{py:func}`~healpix_geo.auto.polygon_coverage` return cells that fall within a polygon, defined by the positions of its vertices.

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
import numpy as np
from healpix_geo.auto import Grid, polygon_coverage

points = np.array(
    [
        [30., 10.],
        [16., 26.],
        [31., 40.],
        [34., 30.],
        [50., 22.],
    ]
)
grid = Grid(level=7, indexing_scheme="nested")

cell_ids, _, fully_covered = polygon_coverage(points, grid)
cell_ids
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
import numpy as np
from matplotlib.collections import PolyCollection
from healpix_geo.auto import vertices

fig, ax = plt.subplots(figsize=(4, 3), dpi=150)

partial_vertices = np.stack(vertices(cell_ids[~fully_covered], grid), axis=-1)
in_bbox_vertices = np.stack(vertices(cell_ids, grid), axis=-1)
ax.add_collection(PolyCollection(partial_vertices, ec="none", fc="lightcoral"))
ax.add_collection(PolyCollection(in_bbox_vertices, ec="0.5", fc="none"))

ax.add_artist(plt.Polygon(points, ec="royalblue", fc="none", lw=2))

ax.set_aspect("equal")
```

+++ {"editable": true, "deletable": true, "slideshow": {"slide_type": ""}}

## MOC

By default, coverage functions return cells at the same refinement level. By passing `flat=False`, they will instead return cells of different sizes (up to a maximum refinement level).

```{code-cell} ipython3
---
editable: true
deletable: true
slideshow:
  slide_type: ''
---
from healpix_geo.zuniq import cone_coverage

max_level = 8
center = (15.0, 20.0)
radius = 5.0

cell_ids, fully_covered = cone_coverage(center, radius, max_level, flat=False)
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
import numpy as np
from matplotlib.collections import PolyCollection
from healpix_geo.zuniq import vertices

fig, ax = plt.subplots(figsize=(4, 3), dpi=150)

partial_vertices = np.stack(vertices(cell_ids[~fully_covered], ellipsoid="sphere"), axis=-1)
in_bbox_vertices = np.stack(vertices(cell_ids, ellipsoid="sphere"), axis=-1)
ax.add_collection(PolyCollection(partial_vertices, ec="none", fc="lightcoral"))
ax.add_collection(PolyCollection(in_bbox_vertices, ec="0.5", fc="none"))

ax.add_artist(plt.Circle(center, radius, ec="royalblue", fc="none", lw=2))

ax.set_aspect("equal")
```
