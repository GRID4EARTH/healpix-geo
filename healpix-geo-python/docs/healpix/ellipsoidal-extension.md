# HEALPix Extension to Ellipsoids

This page outlines an extension of the HEALPix pixelation scheme to ellipsoidal Earth models using an **authalic transformation**: a mapping to a sphere that preserves the surface area.

```{figure} ellipsoid_schema.png
Ellipsoidal to Authalic Mapping
```

## Why Extend HEALPix?

Standard HEALPix is defined on the sphere and provides:

- Hierarchical structure
- Isolatitude grid
- Equal-area pixels

However, many geospatial applications require **ellipsoidal models** (e.g. WGS84). As a consequence, to be viable for these HEALPix needs to be mapped to ellipsoids of revolution.

## How it works

To maintain HEALPix's properties on an ellipsoid, it is mapped to the ellipsoid's **authalic sphere** (a sphere with the same surface area as the ellipsoid). The procedure is:

1. **Authalic Mapping**
   Geodetic latitudes ($\phi$) on the ellipsoid are mapped to authalic latitudes ($\xi$) on the sphere via an area-preserving transformation.

2. **HEALPix on the Sphere**
   Standard HEALPix is applied on the (authalic) sphere using latitude $\xi$ and longitude $\lambda$.

3. **Optional Reverse Mapping**
   Pixel centers or boundaries can be projected back to ellipsoidal coordinates if needed.

Authalic mappings:

- Forward: $\phi \to \xi$ (ellipsoid $\to$ sphere)
- Reverse: $\xi \to \phi$ (sphere $\to$ ellipsoid)

## Ellipsoidal Pixelation

This method results in a **distorted HEALPix grid** on the ellipsoid, preserving equal-area properties. It is ideal for satellite data, climate grids, and DGGS applications that need to account for Earth's flattening.

## See Also

- 📄 [Reference System – HEALPix-Geo Documentation](../reference-system.md)
