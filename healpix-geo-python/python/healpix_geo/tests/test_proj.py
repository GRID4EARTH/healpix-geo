"""Consistency with PROJ's ``+proj=healpix`` projection.

PROJ builds ellipsoidal HEALPix the same way healpix-geo does (geodetic →
authalic latitude → HEALPix projection), so on the projected plane every cell
must be an axis-rotated square, neighbours must share their edges, and PROJ's
inverse must land back in the same cell.
"""

import numpy as np
import pytest

from healpix_geo import ellipsoid as ellipsoid_
from healpix_geo import nested

pyproj = pytest.importorskip("pyproj")

# vertices are moved this fraction of the way towards their cell centre
# before projecting; see `project_cells`
NUDGE = 1e-7
# the nudge shrinks every cell by NUDGE of its size; allow an order of
# magnitude more, relative to the cell size
RTOL = 10 * NUDGE

ELLIPSOIDS = ["unitsphere", "sphere", "WGS84", "GRS80"]


def proj_healpix(ellipsoid, lon_0=0.0):
    params = ellipsoid_.resolve(ellipsoid)
    if "radius" in params:
        shape = f"+R={params['radius']}"
    else:
        shape = f"+a={params['semimajor_axis']} +rf={params['inverse_flattening']}"

    return pyproj.CRS.from_proj4(f"+proj=healpix {shape} +lon_0={lon_0}")


def project_cells(crs, ipix, depth, ellipsoid, lon_0=0.0):
    """Project the vertices and centres of cells onto the HEALPix plane.

    HEALPix is an interrupted projection: a vertex exactly on a cut (a pole,
    a polar facet boundary or the map edge) has no unique image. Nudging every
    vertex towards its own cell centre puts the whole cell on its own facet.
    """
    transformer = pyproj.Transformer.from_crs(crs.geodetic_crs, crs, always_xy=True)
    # the plane is 2πR wide; x(lon_0 + 90°) = πR / 2
    width = 4 * transformer.transform(lon_0 + 90.0, 0.0)[0]

    lon, lat = nested.vertices(ipix, depth, ellipsoid=ellipsoid)
    clon, clat = nested.healpix_to_lonlat(ipix, depth, ellipsoid=ellipsoid)

    lon = clon[:, None] + (lon - clon[:, None] + 180.0) % 360.0 - 180.0
    lon = lon + (clon[:, None] - lon) * NUDGE
    lat = lat + (clat[:, None] - lat) * NUDGE

    x, y = transformer.transform(lon, lat)
    cx, cy = transformer.transform(clon, clat)

    # keep cells crossing the map edge on the side of their centre
    x = x - width * np.round((x - cx[:, None]) / width)

    return x, y, cx, cy, width


@pytest.mark.parametrize("depth", [0, 1, 3])
@pytest.mark.parametrize("ellipsoid", ELLIPSOIDS)
@pytest.mark.parametrize("lon_0", [0.0, 90.0])
def test_cells_are_squares(ellipsoid, depth, lon_0):
    ipix = np.arange(12 * 4**depth, dtype="uint64")
    x, y, cx, cy, width = project_cells(
        proj_healpix(ellipsoid, lon_0), ipix, depth, ellipsoid, lon_0
    )

    side = width / 4 / 2**depth
    np.testing.assert_allclose(x.max(axis=1) - x.min(axis=1), side, rtol=RTOL)
    np.testing.assert_allclose(y.max(axis=1) - y.min(axis=1), side, rtol=RTOL)

    diagonal_02 = np.hypot(x[:, 0] - x[:, 2], y[:, 0] - y[:, 2])
    diagonal_13 = np.hypot(x[:, 1] - x[:, 3], y[:, 1] - y[:, 3])
    np.testing.assert_allclose(diagonal_02, side, rtol=RTOL)
    np.testing.assert_allclose(diagonal_13, side, rtol=RTOL)

    # `healpix_to_lonlat` must give the centre of the square; centres can be 0
    # on the plane, so compare relative to the cell size
    np.testing.assert_allclose(cx, x.mean(axis=1), rtol=0, atol=RTOL * side)
    np.testing.assert_allclose(cy, y.mean(axis=1), rtol=0, atol=RTOL * side)


def test_cells_are_not_squares_for_other_lon_0():
    """Sensitivity check for `test_cells_are_squares`."""
    # the facets only line up with the cells if lon_0 is a multiple of 90°
    depth = 1
    ipix = np.arange(12 * 4**depth, dtype="uint64")
    x, _, _, _, width = project_cells(
        proj_healpix("WGS84", 45.0), ipix, depth, "WGS84", 45.0
    )

    side = width / 4 / 2**depth
    assert not np.allclose(x.max(axis=1) - x.min(axis=1), side, rtol=RTOL)


@pytest.mark.parametrize("ellipsoid", ELLIPSOIDS)
def test_neighbours_share_edges(ellipsoid):
    depth = 3
    ipix = np.arange(12 * 4**depth, dtype="uint64")
    x, y, cx, _, width = project_cells(proj_healpix(ellipsoid), ipix, depth, ellipsoid)
    side = width / 4 / 2**depth

    neighbours = nested.neighbours(ipix, depth, connectivity="edge")
    a, k = np.nonzero(neighbours >= 0)
    b = neighbours[a, k].astype("int64")

    # neighbours in different polar base cells of the same hemisphere share a
    # meridian on the globe, but lie in separate triangles on the plane
    base_a = ipix[a].astype("int64") // 4**depth
    base_b = b // 4**depth
    across_cut = (base_a // 4 != 1) & (base_a // 4 == base_b // 4) & (base_a != base_b)

    xb = x[b] - width * np.round((x[b] - cx[a][:, None]) / width)
    distance = np.hypot(
        xb[:, :, None] - x[a][:, None, :], y[b][:, :, None] - y[a][:, None, :]
    ).min(axis=2)
    # the two vertices of b closest to a are the shared ones
    gap = np.sort(distance, axis=1)[:, 1] / side

    assert across_cut.sum() == 16 * 2**depth  # 8 cuts, both directions
    np.testing.assert_array_less(gap[~across_cut], RTOL)
    assert np.all(gap[across_cut] > 0.1)


@pytest.mark.parametrize("ellipsoid", ELLIPSOIDS)
def test_inverse_projection(ellipsoid):
    depth = 4
    ipix = np.arange(12 * 4**depth, dtype="uint64")
    crs = proj_healpix(ellipsoid)
    x, y, _, _, _ = project_cells(crs, ipix, depth, ellipsoid)

    # centres of the squares on the plane, back through PROJ's inverse
    inverse = pyproj.Transformer.from_crs(crs, crs.geodetic_crs, always_xy=True)
    lon, lat = inverse.transform(x.mean(axis=1), y.mean(axis=1))

    actual = nested.lonlat_to_healpix(lon, lat, depth, ellipsoid=ellipsoid)

    np.testing.assert_equal(actual, ipix)
