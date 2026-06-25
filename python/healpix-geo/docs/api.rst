API reference
=============

This page contains the complete documentation of all the functions and classes available in ``healpix-geo``.

Overview
========

.. toctree::
   :hidden:
   :maxdepth: 2
   :caption: API reference

   api/nested
   api/ring
   api/zuniq
   api/auto

`healpix-geo` is organised in several modules following the **HEALPix indexing scheme** used :

- :doc:`api/nested` : nested scheme
- :doc:`api/ring` : ring scheme
- :doc:`api/zuniq` : Zuniq scheme for Multi-Order Coverage (MOC)
- :doc:`api/auto`: indexing scheme-agnostic API

.. tip::
   **For most of the applications**, use module ``healpix_geo.auto`` with the ``nested`` indexing scheme. It offers the best support for hierarchical operations.

Helpers
~~~~~~~
.. currentmodule:: healpix_geo

.. autosummary::
   :toctree: generated/

   geometry.Bbox

   cartesian_to_lonlat
   lonlat_to_cartesian

Common Parameters
==================

healpix-geo functions shares common parameters :

Geographical coordinates
~~~~~~~~~~~~~~~~~~~~~~~~~

- **lon** (*array-like*) : Longitude(s) in degrees [-180, 180]
- **lat** (*array-like*) : Latitude(s) in degrees [-90, 90]

HEALPix indices
~~~~~~~~~~~~~~~

- **ipix** (*array-like of int*) : Indice(s) of HEALPix cell
- **depth** (*int*) : resolution level [0, 29]

  - 0 = coarser (12 cells)
  - 29 = finer (~1.2 cm)
  - See :doc:`healpix/levels` for the complete table

Ellipsoids
~~~~~~~~~~

- **ellipsoid** (*str*) : Ellipsoidal Model to use

  - ``"WGS84"`` : Standard GPS
  - ``"GRS80"`` : Geodesic System of reference 1980
  - ``"WGS72"`` : Old standard GPS
  - ``"sphere"`` : Perfect sphere

.. tip::
   **Always use** ``ellipsoid="WGS84"`` for real geospatial applications.
   See :doc:`tutorials/ellipsoid_basics` for more details.


Conventions
===========

Return types
~~~~~~~~~~~~

- Functions returns **numpy arrays**
- Coordinates are always in **decimal degrees**
- Indices are always **unsigned integers** (uint64)

Vectorisation
~~~~~~~~~~~~~

All functions are **vectorized** and accept:

- Scalars (automatically converted to arrays)
- 1D NumPy arrays
- Multidimensional NumPy arrays (depending on the function)
