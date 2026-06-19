Module ring
===========

The module `healpix_geo.ring` gives for the ring indexation scheme.

.. currentmodule:: healpix_geo.ring

.. note::
   The ring scheme is principally given for **compatibility**. For new applications, prefer `healpix_geo.nested`.

Coordinates Conversions
~~~~~~~~~~~~~~~~~~~~~~~~~

.. autosummary::
   :toctree: ../generated/

   healpix_to_lonlat
   lonlat_to_healpix
   vertices

Hierarchy
~~~~~~~~~~

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

Distance Calculations
~~~~~~~~~~~~~~~~~~~~~~

.. autosummary::
   :toctree: ../generated/

   angular_distances
