Module zuniq
============

The module `healpix_geo.zuniq` gives functions for the zuniq scheme, utilised for MOC (Multi-Order Coverage).

.. currentmodule:: healpix_geo.zuniq

Conversions
~~~~~~~~~~~

Conversions between schemes nested et zuniq.

.. autosummary::
   :toctree: ../generated/

   from_nested
   to_nested

.. seealso::
   Complete tutorial : :doc:`../tutorials/coordinate_conversion`


Coordinates Conversions
~~~~~~~~~~~~~~~~~~~~~~~~

.. autosummary::
   :toctree: ../generated/

   healpix_to_lonlat
   lonlat_to_healpix
   vertices

Hierarchy and neighbourhood
~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. autosummary::
   :toctree: ../generated/

   kth_neighbourhood

Coverage
~~~~~~~~

Find all cells which intersect a region.

.. autosummary::
   :toctree: ../generated/

   box_coverage
   zone_coverage
   polygon_coverage
   cone_coverage
   elliptical_cone_coverage

.. seealso::
   Complete tutorial : :doc:`../tutorials/coverage_queries`
