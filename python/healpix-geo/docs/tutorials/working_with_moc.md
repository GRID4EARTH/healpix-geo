# Work with MOC

This tutorial explains how to use Multi-Order Coverage maps (MOC) to represent efficiently complex regions.

## zuniq scheme

The MOC is using the **zuniq** scheme which encode depth + ipix in one integer 64-bit.

```python
import healpix_geo
import numpy as np

# Cells nested
ipix_nested = np.array([100, 200, 300])
depth = 8

# Convert to zuniq
zuniq_ids = healpix_geo.nested.to_zuniq(ipix_nested, depth)
print(f"Zuniq IDs: {zuniq_ids}")

# Convert return in nested
ipix_back, depth_back = healpix_geo.zuniq.to_nested(zuniq_ids)
print(f"Nested: depth={depth_back}, ipix={ipix_back}")
```

## Creates a MOC

```python
from healpix_geo.nested import RangeMOCIndex

moc = RangeMOCIndex
```
