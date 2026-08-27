# HEALPix

```{toctree}
---
hidden: true
---
levels
ellipsoidal-extension
```

HEALPix (Hierarchical Equal-Area isoLatitude Pixelisation) was originally defined for use in astronomy by [Gorski et al., 2005]. It has several important properties:

- equal-area: all cells of the same refinement level have exactly the same area
- iso-latitude: all cells of the same refinement level are arranged around rings of the same latitude
- hierarchical: cells are created by recursively and evenly subdividing, which forms a hierarchy of cells

```{figure} healpix_global_indexing.png
:alt: HEALPix nested indexing scheme
:width: 800px
:align: center

The `nested` indexing scheme of HEALPix
```

The latter two properties allow defining the two main indexing schemes:

- `ring`, which assigns identifiers along the iso-latitude rings. Thus, cells on the same latitude ring have identifiers that are "close" numerically (and thus in memory for sorted data).
- `nested`, which assigns identifiers such that sibling cells are close to each other.

To know what cell a identifier refers to, we need two additional parameters: the refinement level (the depth in the hierarchy) and the indexing scheme.

Based on `nested`, there are two more schemes that also encode the refinement level in the cell id:

- `nuniq`, which represents all cells in the hierarchy breadth-first, i.e. cells of a refinement level are close numerically.
- `zuniq`, which represents all cells in the hierarchy depth-first, i.e. descendant cells of a given cell are close numerically.

## Extensions

- [HEALPix Extension to Ellipsoids](ellipsoidal-extension.md): Apply HEALPix to ellipsoidal Earth models via an area-preserving mapping to an authalic sphere.

[Gorski et al., 2005]: https://doi.org/10.1086/427976
