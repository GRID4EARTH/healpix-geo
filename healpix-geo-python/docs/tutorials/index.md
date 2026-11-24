# Tutorials

## To Start

::::{grid} 1 1 2 2
:gutter: 3

:::{grid-item-card} Quickstart
:link: quickstart
:link-type: doc

**Recommended for beginners**

Learn bases of `healpix-geo` in 15 minutes : conversions of coordinates, understand depth, and visualize your first HEALPix cells.
:::

:::{grid-item-card} HEALPix fundamentals and indexing
:link: fundamentals
:link-type: doc

Learn about the fundamentals of HEALPix: refinement level, the different types of indexing.
::::

## Basic Tutorials

::::{grid} 1 1 2 2
:gutter: 3

:::{grid-item-card} Coordinates and cell geometry
:link: coordinates
:link-type: doc

Master the conversions between geographic coordinates (lon/lat) and HEALPix indices.
:::

:::{grid-item-card} Ellipsoids
:link: ellipsoid_basics
:link-type: doc

Understand why and how to use ellipsoid of reference such as WGS84 for precise geodesic calculations.
:::

:::{grid-item-card} Hierarchical indexing
:link: hierarchical_indexing
:link-type: doc

Learn how to navigate the hierarchical indexing (siblings, zoom) and how to find the neighbours of a cell.
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

Use Multi-Order Coverage maps to represent complex regions in a compact way.
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
fundamentals
coordinates
ellipsoid_basics
hierarchical_indexing
coverage_queries
working_with_moc
performance_optimization
```
