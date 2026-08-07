from typing import Protocol, TypedDict

import numpy as np
import numpy.typing as npt


class SphereDict(TypedDict):
    radius: float


class SphereType(Protocol):
    radius: float


class EllipsoidDict(TypedDict):
    semimajor_axis: float
    inverse_flattening: float


class EllipsoidType(Protocol):
    semimajor_axis: float
    inverse_flattening: float


_SphereLike = SphereDict | SphereType
_EllipsoidLike = EllipsoidDict | EllipsoidType

EllipsoidLike = str | _SphereLike | _EllipsoidLike

DepthType = int | npt.NDArray[np.uint8]
