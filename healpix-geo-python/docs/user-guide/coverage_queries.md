# Coverage Queries

`healpix-geo` provides a number of functions to obtain the indices of cells that fall within various geographic regions.
All these functions return three arrays of:

- the indices of cells that fall at least partially in the target region,
- the refinement level of each cell,
- whether each cell is fully covered by the target region.

By default all cells have the same level, the one specified in argument.
By passing `flat=False`, the function will return cells of different sizes (up to a maximum refinement level) so that the cells have the maximum possible size.

:::{note}
Keep in mind that regions are defined on the sphere (or ellipsoid).
:::

## Zone (bounding box)

```{image} coverage_zone.png
:width: 270px
:align: right
```

{py:func}`~healpix_geo.auto.zone_coverage` return cells that fall in a rectangular area defined by its minimum and maximum longitude and latitude.

```{raw} html
<div class="clearer"></div>
```

## Rectangle

```{figure} coverage_box.png
:width: 270px
:align: right
```

{py:func}`~healpix_geo.auto.box_coverage` return cells that fall in a rotated (spherical) rectangular area defined by its center point, size, and rotation angle.

```{raw} html
<div class="clearer"></div>
```

## Cone

```{figure} coverage_cone.png
:width: 225px
:align: right
```

{py:func}`~healpix_geo.auto.cone_coverage` return cells that fall within a cone, _ie_ within a given radius from a center position.
This function relies on approximations: it may return cells that are near the cone but are not covered by it, and some cells might be tagged as partially covered but may really be fully covered. However it guarantees that cells marked as fully covered are really fully covered.

To refine the approximation, computations may be done at a higher refinement level. Computations will be done at level `depth + delta_depth`. By default, `delta_depth` is zero.

```{raw} html
<div class="clearer"></div>
```

## Elliptical cone

```{figure} coverage_elliptical_cone.png
:width: 270px
:align: right
```

{py:func}`~healpix_geo.auto.elliptical_cone_coverage` return cells that fall within an elliptical cone, defined by its center position, its semi minor and major axes, and orientation angle.
Similarly to the cone, this functions relies on approximations.

```{raw} html
<div class="clearer"></div>
```

## Polygon

```{figure} coverage_polygon.png
:width: 210px
:align: right
```

{py:func}`~healpix_geo.auto.polygon_coverage` return cells that fall within a polygon, defined by the positions of its vertices.

```{raw} html
<div class="clearer"></div>
```

## See Also

- {doc}`/tutorials/coverage_queries` - Tutorial on coverage queries
- {doc}`/tutorials/working_with_moc` - Tutorial on MOC
