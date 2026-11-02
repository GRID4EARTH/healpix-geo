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
   api/validation

`healpix-geo` is organised in several modules following the **HEALPix indexing scheme** used :

- :doc:`api/nested` : nested scheme
- :doc:`api/ring` : ring scheme
- :doc:`api/zuniq` : Zuniq scheme for Multi-Order Coverage (MOC)
- :doc:`api/auto`: indexing scheme-agnostic API

.. tip::
   **For most of the applications**, use module ``healpix_geo.auto`` with the ``nested`` indexing scheme. It offers the best support for hierarchical operations.

Base cell topology
~~~~~~~~~~~~~~~~~~

.. currentmodule:: healpix_geo.topology

.. autosummary::
   :toctree: generated/

   base_cell_relationship


Geometry helpers
~~~~~~~~~~~~~~~~
.. currentmodule:: healpix_geo

.. autosummary::
   :toctree: generated/

   geometry.Bbox

   cartesian_to_lonlat
   lonlat_to_cartesian
   vertex_to_lonlat

Ragged output
~~~~~~~~~~~~~

.. currentmodule:: healpix_geo

.. autosummary::
   :toctree: generated/

   RaggedArray


.. _common-parameters:

Common Parameters
=================

All healpix-geo functions accept :external+numpy:term:`array-like<array_like>`
inputs that will be checked and converted to the appropriate dtype. This allows:

- Scalars (automatically converted to 1D arrays)
- N-dimensional NumPy arrays
- (Nested) sequences of arrays or scalars

Some functions will take in multiple arrays. They will not be
:external+numpy:ref:`broadcasted<array-broadcasting-in-numpy>`, and must have
the same shape.

All functions are **vectorized** and will preserve the shape of inputs, excepted
for scalars that will be converted to a 1D array.

HEALPix indices
~~~~~~~~~~~~~~~

- **ipix** (input: *array-like of int*, output: :class:`~numpy.ndarray` of
  :attr:`~numpy.uint64`) : Indices of HEALPix cells. Signed integers are allowed
  as input (*i.e.* array-like of either builtin :class:`int` or
  :class:`numpy.integer`). All inputs will be checked for negative values in
  *zuniq* indexing and for values outside the range :math:`[0, 12\times
  4^{depth}[` otherwise.

- **depth/level** (input: *int or array-like of int*, output:
  :class:`~numpy.ndarray` of :attr:`~numpy.uint8`) : Resolution level in the range
  :math:`[0, 29]`.

  - 0 = coarser (12 cells)
  - 29 = finer (~1.2 cm)
  - See :doc:`healpix/levels` for the complete table

Geographical coordinates
~~~~~~~~~~~~~~~~~~~~~~~~~

- **lon** (input: *array-like of float*, output: :class:`~numpy.ndarray` of
  :attr:`~numpy.float64`) : Longitude(s) in decimal degrees [-180, 180].
  Positive towards East. 0° is Greenwich meridian.
- **lat** (input: *array-like of float*, output: :class:`~numpy.ndarray` of
  :attr:`~numpy.float64`) : Latitude(s) in decimal degrees [-90, 90]. Positive
  towards North. 0° is equator.
- **x**, **y**, **z** (input: *array-like of float*, output:
  :class:`~numpy.ndarray` of :attr:`~numpy.float64`) : Cartesian coordinates, in
  meters.

Ellipsoids
~~~~~~~~~~

- **ellipsoid** (*str*) : Ellipsoidal Model to use

  - ``"WGS84"`` : Standard GPS
  - ``"GRS80"`` : Geodesic System of reference 1980
  - ``"WGS72"`` : Old standard GPS
  - ``"sphere"`` : Perfect sphere

The default is "sphere", in which case most functions will return the same
result as their equivalent from :external+cdshealpix:doc:`cdshealpix<index>`.

.. tip::
   **Always use** ``ellipsoid="WGS84"`` for real geospatial applications.
   See :doc:`tutorials/ellipsoid_basics` for more details.

.. currentmodule:: healpix_geo

.. autosummary::
   :toctree: generated/

   ellipsoid.resolve

Others
~~~~~~

- **num_threads** (*int*) Specifies the number of threads to use for the
  computation. The default (0) means it will choose the number of threads based
  on the RAYON_NUM_THREADS environment variable (if set), or the number of
  logical CPUs (otherwise)
