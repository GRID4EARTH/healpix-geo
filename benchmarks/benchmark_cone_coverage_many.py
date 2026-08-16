"""Compare scalar and batched nested cone coverage.

The default workload models the 15,069-center Level-20 geometry construction
observed in a roughly 600 m healpix-analyse Gaussian-filter patch. Run with:

    python benchmarks/benchmark_cone_coverage_many.py
"""

from __future__ import annotations

import argparse
import math
import time

import numpy as np

from healpix_geo import nested


def make_centers(count: int) -> np.ndarray:
    side = math.ceil(math.sqrt(count))
    # About one Level-20 cell (six metres) between centers near Paris.
    spacing_lat = 6.0 / 111_320.0
    spacing_lon = spacing_lat / math.cos(math.radians(48.86))
    axis = np.arange(side, dtype=np.float64) - (side - 1) / 2
    lon, lat = np.meshgrid(2.35 + axis * spacing_lon, 48.86 + axis * spacing_lat)
    return np.column_stack((lon.ravel(), lat.ravel()))[:count]


def timed(function):
    start = time.perf_counter()
    result = function()
    return time.perf_counter() - start, result


def assert_matches_scalar(scalar_rows, batched):
    offsets, cell_ids, depths, fully_covered = batched
    assert len(offsets) == len(scalar_rows) + 1
    for index, expected in enumerate(scalar_rows):
        start, end = offsets[index : index + 2]
        np.testing.assert_array_equal(cell_ids[start:end], expected[0])
        np.testing.assert_array_equal(depths[start:end], expected[1])
        np.testing.assert_array_equal(fully_covered[start:end], expected[2])


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--count", type=int, default=15_069)
    parser.add_argument("--depth", type=int, default=20)
    parser.add_argument("--radius-metres", type=float, default=100.0)
    args = parser.parse_args()

    centers = make_centers(args.count)
    radius_degrees = args.radius_metres / 111_320.0
    kwargs = {"ellipsoid": "WGS84", "flat": True}

    scalar_seconds, scalar_rows = timed(
        lambda: [
            nested.cone_coverage(center, radius_degrees, args.depth, **kwargs)
            for center in centers
        ]
    )
    one_thread_seconds, one_thread = timed(
        lambda: nested.cone_coverage_many(
            centers,
            radius_degrees,
            args.depth,
            num_threads=1,
            **kwargs,
        )
    )
    eight_thread_seconds, eight_threads = timed(
        lambda: nested.cone_coverage_many(
            centers,
            radius_degrees,
            args.depth,
            num_threads=8,
            **kwargs,
        )
    )

    assert_matches_scalar(scalar_rows, one_thread)
    assert_matches_scalar(scalar_rows, eight_threads)

    print(
        f"centers: {args.count:,}; depth: {args.depth}; radius: {args.radius_metres:g} m"
    )
    print(f"scalar Python loop: {scalar_seconds:.6f} s")
    print(
        f"batch, one thread: {one_thread_seconds:.6f} s "
        f"({scalar_seconds / one_thread_seconds:.2f}x)"
    )
    print(
        f"batch, eight threads: {eight_thread_seconds:.6f} s "
        f"({scalar_seconds / eight_thread_seconds:.2f}x)"
    )
    print("correctness: all batched rows exactly match scalar results")


if __name__ == "__main__":
    main()
