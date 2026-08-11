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

# Advanced coordinate conversion

This tutorial delves deeper into conversions between geographic coordinates and HEALPix indices, exploring the different indexing schemes.

## HEALPix indexing schemes

HEALPix supports three different indexing schemes:

- nested
- ring
- zuniq

## Conversions from geographic coordinates index

### Nested and Ring

```{code-cell} python
import numpy as np
from healpix_geo.nested import lonlat_to_healpix as nested_lonlat
from healpix_geo.ring import lonlat_to_healpix as ring_lonlat

# Same point, two schemes
lon, lat = np.array([0.0]), np.array([45.0])
depth = 8

ipix_nested = nested_lonlat(lon, lat, depth, "WGS84")
ipix_ring = ring_lonlat(lon, lat, depth, "WGS84")

print(f"Point (0°, 45°) at depth={depth}:")
print(f"  Nested: {ipix_nested[0]}")
print(f"  Ring:   {ipix_ring[0]}")
```

:::{warning}
The nested and ring indices **are not interchangeable**. The same point has different indices depending on the scheme!
:::

### Nested and Zuniq

```{code-cell} python
import healpix_geo
import numpy as np

# Nested → Zuniq
ipix_nested = 349440
depth = 8
zuniq_id = healpix_geo.nested.to_zuniq(ipix_nested, depth)

print(f"Nested (depth={depth}, ipix={ipix_nested}) → Zuniq: {zuniq_id}")

# Zuniq → Nested
ipix_back, depth_back = healpix_geo.zuniq.to_nested(zuniq_id)
print(f"Zuniq {zuniq_id} → Nested (depth={depth_back}, ipix={ipix_back})")
```

### Visual comparison

Let see how the different schemes are organising cells :

```{code-cell} python
---
tags: [hide-input]
---
import matplotlib.pyplot as plt
from healpix_geo.nested import vertices as nested_vertices
from healpix_geo.ring import vertices as ring_vertices

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(16, 6))

depth = 2
n_cells = 12 * 4**depth

# Sample cells
sample_cells = range(0, n_cells, max(1, n_cells // 50))

# Nested
for ipix in sample_cells:
    lons, lats = nested_vertices(np.array([ipix]), depth, "WGS84")
    lons_closed = np.append(lons[:, 0], lons[0, 0])
    lats_closed = np.append(lats[:, 0], lats[0, 0])

    ax1.plot(lons_closed, lats_closed, "b-", linewidth=0.5, alpha=0.5)
    ax1.text(
        lons[:, 0].mean(),
        lats[:, 0].mean(),
        str(ipix),
        ha="center",
        va="center",
        fontsize=6,
    )

ax1.set_title(f"Nested Scheme (depth={depth})", fontsize=14)
ax1.set_xlabel("Longitude (°)")
ax1.set_ylabel("Latitude (°)")
ax1.grid(True, alpha=0.3)

# Ring
for ipix in sample_cells:
    lons, lats = ring_vertices(np.array([ipix]), depth, "WGS84")
    lons_closed = np.append(lons[:, 0], lons[0, 0])
    lats_closed = np.append(lats[:, 0], lats[0, 0])

    ax2.plot(lons_closed, lats_closed, "r-", linewidth=0.5, alpha=0.5)
    ax2.text(
        lons[:, 0].mean(),
        lats[:, 0].mean(),
        str(ipix),
        ha="center",
        va="center",
        fontsize=6,
    )

ax2.set_title(f"Ring Scheme (depth={depth})", fontsize=14)
ax2.set_xlabel("Longitude (°)")
ax2.set_ylabel("Latitude (°)")
ax2.grid(True, alpha=0.3)

plt.tight_layout()
plt.show()
```

## Choice Table

| Need                    | Recommended Scheme |
| ----------------------- | ------------------ |
| General Application     | **nested**         |
| Hierarchical Navigation | **nested**         |
| Spherical harmonics     | **ring**           |
| MOC                     | **zuniq**          |
| Order by latitude       | **ring**           |

## Next Steps

::::{grid} 1 1 2 2
:gutter: 2

:::{grid-item-card} Cover Requests
:link: coverage_queries
:link-type: doc
:::

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

::::
