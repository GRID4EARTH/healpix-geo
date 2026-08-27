# Reference system

Geographic data can be expressed in different coordinate systems:

- **3D cartesian** : Coordinates (x, y, z) in space
- **Spherical** : Simple model with constant radius
- **Ellipsoidal** : Accurate model with flattening at the poles
- **Projected** : Projection onto a plane (e.g., Mercator, UTM)

3D cartesian coordinates are the simplest coordinates, but rather tricky to use to describe coordinates on earth's curved surface.

Instead, the earth's surface is commonly approximated in first order as a **sphere**, and in second order as an **ellipsoid**.

## Ellipsoidal coordinates

In coordinate systems based on a ellipsoid of revolution, geographic coordinates of points are expressed as tuples of longitude, latitude, and height above ellipsoid: {math}`(\lambda, \phi, h)`. This is much like the spherical coordinate system, except that the local normal on the ellipsoid in general does not pass through the intersection of the rotational axis and the equatorial plane (the ellipsoid's center).

```{figure} ellipsoidal_latitudes.png
The relation between the geographic latitude and the various auxiliary latitudes.

The point $P$ has the coordinates $(\lambda, \phi, h)$ (only the latitude is shown). Setting the height to $0$ results in $P_0$, which is used to derive all auxiliary latitudes.
```

In addition to the geographic latitude (also called the geodetic, astronomic, or common latitude), there are a range of auxiliary latitudes[^1], including:

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

Karney, 2023[^2], describes conversion formulas that reduce the numerical round-off errors.

[^1]: https://doi.org/10.3133/pp1395

[^2]: https://doi.org/10.1080/00396265.2023.2217604
