#!/usr/bin/env xpython

import sys

print("starting to run script")
print(sys.version, flush=True)

import numpy as np

import healpix_geo

level = 2
cell_ids = np.arange(12 * 4**level)

print(
    f"result: {healpix_geo.nested.healpix_to_lonlat(cell_ids, level, ellipsoid='sphere')}"
)
