
.. module:: healpix_geo.ring

Module ring
===========

The module `healpix_geo.ring` gives for the ring indexation scheme.

.. currentmodule:: healpix_geo.ring

.. note::
   The ring scheme is principally given for **compatibility**. For new applications, prefer `healpix_geo.nested`.

Coordinate Conversions
~~~~~~~~~~~~~~~~~~~~~~

.. autosummary::
   :toctree: ../generated/

   healpix_to_lonlat
   lonlat_to_healpix
   healpix_to_cartesian
   cartesian_to_healpix

.. seealso::
   :doc:`tutorial </tutorials/coordinates>` and :doc:`user-guide </user-guide/coordinates>`

Cell geometry
~~~~~~~~~~~~~
.. autosummary::
   :toctree: ../generated/

   vertices
   vertex_indices

.. seealso::
   :doc:`tutorial </tutorials/coordinates>` and :doc:`user-guide </user-guide/coordinates>`

Indexing Scheme Conversions
~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. autosummary::
   :toctree: ../generated/

   to_nested
   to_zuniq

Interpolation
~~~~~~~~~~~~~

Interpolation from HEALPix to geographic coordinates.

.. autosummary::
   :toctree: ../generated/

   bilinear_interpolation

Hierarchy
~~~~~~~~~~

.. autosummary::
   :toctree: ../generated/

   kth_neighbours
   kth_neighbourhood

.. seealso::
   :doc:`tutorial </tutorials/hierarchical_indexing>` and :doc:`user-guide </user-guide/hierarchical_indexing>`

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
   :doc:`tutorial </tutorials/coverage_queries>` and :doc:`user-guide </user-guide/coverage_queries>`

Distance Calculations
~~~~~~~~~~~~~~~~~~~~~~

.. autosummary::
   :toctree: ../generated/

   angular_distances
   geodesic_distances
