Module auto
===========
The `healpix_geo.auto` module provides a indexing scheme-agnostic API.

.. currentmodule:: healpix_geo.auto

Container
---------
.. autosummary::
   :toctree: ../generated/

   Grid

Coordinate Conversions
----------------------

.. autosummary::
   :toctree: ../generated/

   healpix_to_lonlat
   lonlat_to_healpix
   healpix_to_cartesian
   cartesian_to_healpix
   vertices

Interpolation
-------------
Interpolation from HEALPix to geographic coordinates.

.. autosummary::
   :toctree: ../generated/

   bilinear_interpolation

Hierarchy and neighbourhood
----------------------------

.. autosummary::
   :toctree: ../generated/

   kth_neighbours
   kth_neighbourhood

Coverage requests
-----------------

.. autosummary::
   :toctree: ../generated/

   box_coverage
   zone_coverage
   polygon_coverage
   cone_coverage
   elliptical_cone_coverage
