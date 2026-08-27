import numpy as np


def q(ϕ, e):
    # from USGS Professional Paper 1395, "Map projections – A working manual" by John P. Snyder
    return (1 - e**2) * (
        np.sin(ϕ) / (1 - e**2 * np.sin(ϕ) ** 2)
        - (1 / (2 * e)) * np.log((1 - e * np.sin(ϕ)) / (1 + e * np.sin(ϕ)))
    )


def authalic_radius(a, e):
    q_p = q(np.pi / 2, e)
    return a * np.sqrt(q_p / 2)


def convert(geographic, e):
    q_ = q(geographic, e)
    q_p = q(np.pi / 2, e)
    xi = np.arcsin(q_ / q_p)

    return xi
