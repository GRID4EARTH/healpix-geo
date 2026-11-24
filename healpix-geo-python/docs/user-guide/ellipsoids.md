# Ellipsoids and Geodesy

## Ellipsoidal HEALPix

Standard HEALPix is defined on the sphere. However, as the Earth is not a perfect sphere but an ellipsoid flattened at the poles, some applications (such as satellite data or climate grid) require **ellipsoidal models** (e.g. WGS84) to account for Earth's flattening.

To maintain HEALPix's equal-area property on an ellipsoid, HEALPix operations are done on the ellipsoid's **authalic sphere**: a sphere with the same surface area as the ellipsoid. In effect, this results in a distorted HEALPix grid that retains its equal-area property. The process is as follows:

- **Authalic Mapping**:
  Geodetic latitudes ($\phi$) on the ellipsoid are mapped to authalic latitudes ($\xi$) on the sphere via an area-preserving transformation.

- **HEALPix on the Sphere**:
  Standard HEALPix is applied on the authalic sphere using latitude $\xi$ and longitude $\lambda$.

- **Reverse Mapping**:
  Pixel centers or boundaries can be projected back to geodetic coordinates if needed.

```{figure} ellipsoid_schema.png
:alt: Ellipsoidal to authalic mapping diagram

Ellipsoidal to Authalic Mapping
```

## Specifying the ellipsoid

In healpix-geo, the ellipsoidal model to use can be specified via the `ellipsoid` keyword argument to the {py:class}`.Grid` object if you are using the {py:mod}`.auto` module, or directly in the function otherwise.

The ellipsoidal model can be specified either by name or by parameters. Available names are those built-in in the [geodesy rust crate](https://github.com/busstoptaktik/geodesy/blob/f9090b8c91f401892a93979f100fa4d987eb0836/src/ellipsoid/constants.rs#L6-L54).
Some of the available names are:
:::{table}

| name   |                                        |
| ------ | -------------------------------------- |
| sphere | Normal sphere (r=6370997), **default** |
| WGS84  | WGS 84 (current standard)              |
| WGS72  | WGS 72 (old standard)                  |
| GRS80  | GRS 80                                 |

:::

The ellipsoidal model can also be specified by a dictionary of parameters:

- `"radius"` (in meters) for a spherical model,
- `"semimajor_axis"` (in meters) and `"inverse_flattening"` for an ellipsoidal model.

For retro-compatibility reasons, the default is `"sphere"` in which case the result is identical to standard healpix [cdshealpix](https://cds-astro.github.io/cds-healpix-python).

:::{tip}
For most real-world geospatial applications, use **WGS84**, as it is the current global geodetic standard.
Use a spherical model (**sphere**) only when high accuracy is not required.
:::

## Ellipsoidal coordinates

Here are more details on the authalic coordinates derivation.

In coordinate systems based on a ellipsoid of revolution, geographic coordinates of points are expressed as tuples of longitude, latitude, and height above ellipsoid: $(\lambda, \phi, h)$. This is much like the spherical coordinate system, except that the local normal on the ellipsoid in general does not pass through the intersection of the rotational axis and the equatorial plane (the ellipsoid's center).

```{figure} ellipsoidal_latitudes.png
:alt: Scheme of the relation between geographic latitude and various auxiliary latitudes.

The relation between the geographic latitude and the various auxiliary latitudes.

The point $P$ has the coordinates $(\lambda, \phi, h)$ (only the latitude is shown). Setting the height to $0$ results in $P_0$, which is used to derive all auxiliary latitudes.
```

In addition to the geographic latitude (also called the geodetic, astronomic, or common latitude), there are a range of auxiliary latitudes ([Snyder (1987)](#snyder_1987)), including:

- the parametric (or reduced) latitude $\beta$, which is the result of stretching the semi-minor axis $b$ to the length of the semi-major axis $a$, resulting in a sphere. This stretching displaces $P_0$ to $P_\beta$. The conversion is
  ```{math}
  \tan{\beta} = \frac{b}{a}\tan{\phi} = (1 - f)\tan{\phi} = \sqrt{1 - e^2}\tan{\phi}
  ```
- the geocentric latitude $\theta$, which is the spherical latitude for the local radius at $P_0$ (the distance between the center and $P_0$). The conversion is
  ```{math}
  \tan{\theta} = \frac{b^2}{a^2} \tan{\phi} = (1 - e^2)\tan{\phi}
  ```
- the authalic latitude $\xi$, which is the result of stretching $b$ and compressing $a$ such that the resulting sphere (the "authalic sphere") has the same surface area as the ellipsoid. In the process, $P_0$ is displaced to $P_\xi$ to keep the surface area of faces on the authalic sphere the same as on the ellipsoid.

  The conversion is

  ```{math}
  q(\phi) &= \frac{(1-e^2)\sin{\phi}}{1 - e^2\sin^2{\phi}} - \frac{1-e^2}{2e} \ln{\left(\frac{1-e\sin{\phi}}{1 + e\sin{\phi}}\right)} \\
  \sin{\xi} &= \frac{q(\phi)}{q(\frac{\pi}{2})}
  ```

  And the inverse has to be computed iteratively or through a series expansion.

Note that $\beta$ and $\xi$ are latitudes for auxiliary spheres.

````{table} Ellipsoidal latitudes for WGS84 ($a = 6378137 m, f^{-1} = 298.257223563$)

```{include} latitude_diff_table.md
```

````

[Karney (2023)](#karney_2024) describes conversion formulas that reduce the numerical round-off errors.

## See Also

- {doc}`/tutorials/ellipsoid_basics` - Tutorial on ellipsoids

## References

(snyder_1987)=

- Snyder, J. (1987), _"Map projections: A working manual"_, U.S. Geological Survey Professional Paper 1395, Report: ix, 385 p., <doi:10.3133/pp1395>
  (karney_2024)=
- Karney, C. F. F. (2024), _"On auxiliary latitudes"_, Survey Review, 56(395), 165–180. <doi:10.1080/00396265.2023.2217604>
