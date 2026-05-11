# Installation

This guide explains you how to install `healpix-geo` on your system.

## Prerequisites

- **Python** : Version >= 3.10
- **NumPy** : Automatically install with `healpix-geo`

### Via conda

::::{tab-set}

:::{tab-item} conda

```bash
conda install -c conda-forge healpix-geo
```

:::

:::{tab-item} mamba

```bash
mamba install -c conda-forge healpix-geo
```

:::

:::{tab-item} pixi

```bash
pixi add healpix-geo
```

:::

::::

### Via pip

::::{tab-set}

:::{tab-item} pip

```bash
pip install healpix-geo
```

:::

:::{tab-item} uv

```bash
uv add healpix-geo
```

:::

::::

:::{note}
Pre-compiled wheels are available for the following platforms :

- **Linux** : x86_64, aarch64
- **macOS** : x86_64 (Intel), arm64 (Apple Silicon)
- **Windows** : x86_64
  :::

## Installation from sources

### Prerequisite

- **Rust** : Version >= 1.70 ([installation](https://rustup.rs/))
- **Python** : Version >=3.9
- **pixi** : ([installation](https://pixi.sh/))

### Steps

1. **Clone the depository** :

```bash
git clone https://github.com/GRID4EARTH/healpix-geo.git
cd healpix-geo
```

2. **Build wheels** :

Option 1 - build wheels for all supported python versions :

```bash
pixi run build-all-wheels
```

Option 2 - build wheel for python=3.13 :

```bash
pixi run -e py313 build-wheel
```

3. **Install the appropriate wheel** :

```bash
pip install ./target/wheels/healpix-geo-<version>-cp313-cp313-<platform>.whl
```

## Uninstallation

To uninstall `healpix-geo` :

```bash
pip uninstall healpix-geo
# or
conda remove healpix-geo
```
