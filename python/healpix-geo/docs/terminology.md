# Terminology

## General terms

```{glossary}
HEALPix
    Hierarchical Equal Area isoLatitude Pixelization. A spherical tiling system that divides a sphere into cells of equal area, organized hierarchically.

Cell
    An individual region of the HEALPix tiling. Each cell is a spherical quadrilateral with 4 vertices.

Depth
    The resolution level in the HEALPix hierarchy. Depth 0 = 12 cells, depth N = 12 × 4^N cells. Range: [0, 29].

Index, ipix
    The unique numeric identifier of a HEALPix cell at a given level. Type: 64-bit unsigned integer (uint64).
```

## Indexing scheme

```{glossary}
Nested
    Hierarchical indexing scheme where child cells have consecutive indices. Optimized for hierarchical operations. **Recommended by default.**

Ring
    Latitude ring indexing scheme. Cells on the same latitude ring have similar indices. Used primarily for compatibility.

Zuniq
    An indexing scheme that encodes both the depth and the index into a single 64-bit integer. Used for Multi-Order Coverage (MOC).

Nuniq
   A variant of zuniq with breadth-first ordering (by level). Less used than zuniq.
```

## Geometry and coordinates

```{glossary}
Longitude
    Range: [-180°, +180°]. 0° = Greenwich meridian. Positive towards the East.

Latitude
    Range: [-90°, +90°]. 0° = ecuador. Positive towards the North.

Geodetic latitude, Geographic latitude
    The standard latitude used in cartography, measured perpendicular to the ellipsoid. This is the latitude returned by GPS devices. Symbol: φ (phi).

Geocentric latitude
    Latitude measured from the center of the ellipsoid. Symbol: θ (theta).

Parametric latitude, Reduced latitude
    Auxiliary latitude obtained by "stretching" the ellipsoid into a sphere. Symbol: β (beta)

Authalic latitude
    Latitude on a sphere with the same area as the ellipsoid. Used by HEALPix for ellipsoids. Symbol: ξ (xi).

Vertices
    The coins of an HEALPix cell.

Center of a cell
    The geometric center of a HEALPix cell. Coordinates returned by `healpix_to_lonlat`.
```

## Ellipsoids and geodesy

```{glossary}
Ellipsoid
    Mathematical model of the Earth's shape, described by two radius (equatorial and polar). More accurate than a sphere.

Sphere
    Simplified model where the Earth has a constant radius. Less precise than an ellipsoid but faster calculations.

Flattening
    Measure of the difference between the ellipsoid and a sphere. Symbol: f.

Eccentricity
    Another measure of flatness. Symbol: e. Related to f by: e² = 2f - f².

ellipsoid-like
    An ellipsoid specification. Can be either:

    - The name of the ellipsoid as a {py:class}`str`. For a complete list of known ellipsoids, see [the `geodesy` create](https://github.com/busstoptaktik/geodesy/blob/f9090b8c91f401892a93979f100fa4d987eb0836/src/ellipsoid/constants.rs#L6-L54).
    - A {py:class}`dict`, with either a ``"radius"`` item for spheres or ``"semimajor_axis"`` and ``"inverse_flattening"`` for ellipsoids. All items need to be {py:class}`float`s.
    - A class with a ``"radius"`` attribute for spheres or ``"semimajor_axis"`` and ``"inverse_flattening"`` attributes for ellipsoids. All attributes need to be {py:class}`float`s.

    If an object or {py:class}`dict` could be interpreted as both a sphere and an ellipsoid, the ellipsoid will be preferred.
```

## Hierarchy

```{glossary}
Parent
    At depth N, the parent of a cell is the cell of level N-1 which contains it.

Children
    At depth N, a cell has 4 children at depth N+1 which subdivide it.

Siblings
    The 4 children cells of a same parent.

Neighbourhood
    The set of adjacent cells. A neighborhood of order k includes cells at a distance of k.

k-th neighborhood, Neighborhood of order k
    - k=1: direct neighbors (share an edge)
    - k=2: neighbors at a distance of 2
    - etc.

Zoom
    Navigation between resolution levels. `zoom_to(ipix, depth_from, depth_to)` converts cells from one level to another.
```
