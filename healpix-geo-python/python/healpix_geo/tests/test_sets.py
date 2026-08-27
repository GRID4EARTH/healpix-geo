import numpy as np
import pytest

import healpix_geo


@pytest.mark.parametrize("depth", (3, 5))
@pytest.mark.parametrize("size", (1, 3))
@pytest.mark.parametrize("center", (3, 41))
def test_internal_boundary(depth, center, size):
    center = np.array([center], dtype="uint64")
    domain = healpix_geo.nested.kth_neighbourhood(ipix=center, depth=depth, ring=size)
    cells = np.unique(domain, sorted=True)
    cells = cells[cells != -1].astype("uint64")

    expanded_domain = healpix_geo.nested.kth_neighbourhood(
        ipix=cells, depth=depth, ring=1
    )
    expanded = np.unique(expanded_domain, sorted=True)
    expanded = expanded[expanded != -1].astype("uint64")

    expected = np.setdiff1d(expanded, cells)

    actual = healpix_geo.nested.internal_boundary(depth, expanded)

    np.testing.assert_equal(np.sort(actual), expected)
