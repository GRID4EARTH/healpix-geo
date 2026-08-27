---
jupytext:
  formats: md:myst
  text_representation:
    extension: .md
    format_name: myst
kernelspec:
  name: python3
  display_name: Python 3
---

# Performance optimisations

This tutorial resents techniques for optimised performances of HEALPix calculations.

## Vectorisation

Always prefer vectorised operations :

```{code-cell} python
import time
import numpy as np
from healpix_geo.nested import lonlat_to_healpix

N = 100
lon = np.random.uniform(-180, 180, N)
lat = np.random.uniform(-90, 90, N)

# Good : vectorised
t0 = time.perf_counter()
ipix = lonlat_to_healpix(lon, lat, depth=10, ellipsoid="WGS84")
t_vec = time.perf_counter() - t0

# Bad : loop
t0 = time.perf_counter()
ipix_list = []
for i in range(N):
    ipix_list.append(lonlat_to_healpix(lon[i : i + 1], lat[i : i + 1], 10, "WGS84")[0])
t_loop = time.perf_counter() - t0

print(f"Vectorised : {t_vec * 1000:.1f} ms  ({N:,} points)")
print(f"Loop       : {t_loop * 1000:.1f} ms  ({N:,} points)")
print(f"Speedup    : {t_loop / t_vec:.0f}×  faster with vectorisation")
```

## Multi-threading

The `num_threads` parameter controls parallel execution.

```python
import numpy as np
from healpix_geo.nested import lonlat_to_healpix

# Automatic (use all available CPU cores)
ipix = lonlat_to_healpix(lon, lat, depth=10, ellipsoid="WGS84", num_threads=0)

# Use 4 threads
ipix = lonlat_to_healpix(lon, lat, depth=10, ellipsoid="WGS84", num_threads=4)

# Sequential execution (single thread)
ipix = lonlat_to_healpix(lon, lat, depth=10, ellipsoid="WGS84", num_threads=1)
```

:::{tip}

- num_threads=0 uses all available CPU cores.
- num_threads=1 disables parallelism.
- num_threads>1 uses the specified number of threads.
  :::
