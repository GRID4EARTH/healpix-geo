import healpix_geo
import numpy as np


def test_nested_lonlat_to_healpix():
    level = 2
    cell_ids = np.arange(12 * 4**level)
    lon, lat = healpix_geo.nested.healpix_to_lonlat(cell_ids, level)

    assert lon.shape == cell_ids.shape
