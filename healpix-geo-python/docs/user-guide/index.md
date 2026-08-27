# User Guide

This guide explains the fundamental concepts of healpix-geo and how to use them effectively.

## Guide organisation

### To start

If you begin with `healpix-geo` and HEALPix, read in the order :

1. {doc}`concepts` - Understand HEALPix and healpix-geo
2. {doc}`ellipsoids` - Reference system
3. {doc}`coordinates_systems` - Coordinates system

### Advanced functionalities

When the basis are acquired :

4. {doc}`hierarchical_indexing` - Hierarchical structure and navigation
5. {doc}`coverage_queries` - Cover request
6. {doc}`performance` - Optimisation of calculation

## Chapters overview

::::{grid} 1 1 2 2
:gutter: 3

:::{grid-item-card} Fundamental concepts
:link: concepts
:link-type: doc

Understanding what HEALPix is, why to use it, and the basic principles of equal-area spherical tiling.

:::

:::{grid-item-card} Ellipsoids
:link: ellipsoids
:link-type: doc

Earth models, reference ellipsoids, and their impact on your calculations.

:::

:::{grid-item-card} Coordinate systems
:link: coordinates_systems
:link-type: doc

Conversions between longitude/latitude and HEALPix indices. Nested schemes,
ring, and zuniq.

:::

:::{grid-item-card} Hierarchical indexing
:link: hierarchical_indexing
:link-type: doc

Navigating the hierarchy: parents, children, neighbors, and multiple resolution levels.

:::

:::{grid-item-card} Coverage queries
:link: coverage_queries
:link-type: doc

Find all cells that intersect a region: circles, polygons, bounding boxes.

:::

:::{grid-item-card} Performance
:link: performance
:link-type: doc

Optimize your calculations: vectorization, multi-threading, and best practices for processing large volumes.

:::

::::

## Conventions

### Notation

In this guide, we are using the following conventions :

- `lon`, `lat` : Longitude and latitude
- `ipix` : Healpix cell index
- `depth` : Resolution level
- `ellipsoid` : Ellipsoidal model

## Quick Glossary

| Term          | Definition                                               |
| ------------- | -------------------------------------------------------- |
| **HEALPix**   | Hierarchical Equal Area isoLatitude Pixelization         |
| **Depth**     | Resolution level (0 = coarser, 29 = finer)               |
| **Nested**    | Hierarchical indexing scheme                             |
| **Ring**      | Latitude ring indexing scheme                            |
| **Zuniq**     | Indexing scheme for MOC                                  |
| **MOC**       | Multi-Order Coverage - compact representation of regions |
| **Ellipsoid** | Mathematical model of the shape of the Earth             |
| **WGS84**     | World Geodetic System 1984                               |

:::{seealso}
For a complete glossary, see {doc}`../terminology`.
:::

## Guide of module choice

| Module     | When to use it                                                | Principal functionalities                               |
| ---------- | ------------------------------------------------------------- | ------------------------------------------------------- |
| **nested** | General applications, Hierarchical requests, Multi-resolution | Coordinates conversions, Coverage, Hierarchy, Distances |
| **ring**   | Compatibility legacy, Specific order requested                | Coordinates conversions, Distances, Limited neighbours  |
| **zuniq**  | Work with MOC, Interoperability                               | Conversions nested ↔ zuniq, Cell Coordinates            |

```{toctree}
---
hidden: true
---
concepts
ellipsoids
coordinates_systems
hierarchical_indexing
coverage_queries
performance
```
