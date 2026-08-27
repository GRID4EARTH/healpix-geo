# Tutorials

## To Start

::::{grid} 1 1 2 2
:gutter: 3

:::{grid-item-card} Quick Start
:link: quickstart
:link-type: doc

**Recommended for beginners**

Learn bases of `healpix-geo` in 15 minutes : conversions of coordinates, understand depth, and visualised your first HEALPix cells.
:::

:::{grid-item-card} Ellipsoids
:link: ellipsoid_basics
:link-type: doc

Understand why and how to use ellipsoid of reference such as WGS84 for precise geodesic calculations.

:::

::::

## Basic Tutorials

::::{grid} 1 1 2 2
:gutter: 3

:::{grid-item-card} Coordinates Conversion
:link: coordinate_conversion
:link-type: doc

Master the conversions between geographic coordinates (lon/lat) and HEALPix indices, with the different indexing schemes.

:::

:::{grid-item-card} Cover requests
:link: coverage_queries
:link-type: doc

Learn to find all HEALPix cells that intersect a region.
:::

::::

## Advanced Tutorials

::::{grid} 1 1 2 2
:gutter: 3

:::{grid-item-card} Working with MOC
:link: working_with_moc
:link-type: doc

Use les Multi-Order Coverage maps to represent complex regions in a compact way.

:::

:::{grid-item-card} Performance optimisation
:link: performance_optimization
:link-type: doc

Techniques for optimizing performance: vectorization, multi-threading, and best practices.

:::

::::

```{toctree}
---
hidden: true
---
quickstart
ellipsoid_basics
coordinate_conversion
coverage_queries
working_with_moc
performance_optimization
```
