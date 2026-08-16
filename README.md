[![Rust CI](https://github.com/GRID4EARTH/healpix-geo/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/GRID4EARTH/healpix-geo/actions/workflows/rust-ci.yml)
[![Python CI](https://github.com/GRID4EARTH/healpix-geo/actions/workflows/python-ci.yml/badge.svg)](https://github.com/GRID4EARTH/healpix-geo/actions/workflows/python-ci.yml)
[![Docs](https://readthedocs.org/projects/healpix-geo/badge/?version=latest)](https://healpix-geo.readthedocs.io/)
[![Formatted with black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/python/black)
[![Available on pypi](https://img.shields.io/pypi/v/healpix-geo.svg)](https://pypi.python.org/pypi/healpix-geo/)
[![PyPI Downloads](https://pepy.tech/badge/healpix-geo)](https://pepy.tech/projects/healpix-geo)
[![conda-forge](https://anaconda.org/conda-forge/healpix-geo/badges/downloads.svg)](https://anaconda.org/conda-forge/healpix-geo)
[![DOI](https://zenodo.org/badge/930370213.svg)](https://zenodo.org/badge/latestdoi/930370213)

# `healpix-geo`

`healpix-geo` provides HEALPix algorithms and bindings for geoscience applications. It
builds on [`cds-healpix-rust`](https://github.com/cds-astro/cds-healpix-rust) and complements
[`cds-healpix-python`](https://github.com/cds-astro/cds-healpix-python).

The package is part of the [GRID4EARTH](https://github.com/GRID4EARTH) ecosystem. GRID4EARTH
development is primarily supported by activities funded by the European Space Agency (ESA),
and external contributions are welcome.

## Installation

Get it from `conda-forge` (recommended):

```{sh}
conda install -c conda-forge healpix-geo  # with conda
pixi add healpix-geo  # with pixi
```

Or from PyPI:

```{sh}
pip install healpix-geo  # with pip
uv add healpix-geo  # with uv
```

For more information, see the [documentation](https://healpix-geo.readthedocs.io/en/latest).

## Related GRID4EARTH repositories

- [`healpix-analyse`](https://github.com/GRID4EARTH/healpix-analyse) uses `healpix-geo` for
  HEALPix-based analysis workflows.
- See the [GRID4EARTH organization](https://github.com/GRID4EARTH) for other related projects.

## Development and contributing

Bug reports, feature requests, and contributions are welcome. See
[`CONTRIBUTING.md`](CONTRIBUTING.md) for the contribution process and
[`DEVELOPMENT.md`](DEVELOPMENT.md) for the lightweight branch and cross-repository development
workflow.

## Funding and acknowledgements

**Funded by ESA, built by a European consortium.**

ESA Contract: `4000147951/25/I-NS` — Technical Officer: Vincent Dumoulin.

We also thank all external contributors and the open-source projects on which `healpix-geo`
depends.
