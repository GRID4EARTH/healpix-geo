
.. module:: healpix_geo.zuniq

Module zuniq
============

The module `healpix_geo.zuniq` gives functions for the zuniq scheme, utilised for MOC (Multi-Order Coverage).

.. currentmodule:: healpix_geo.zuniq

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

   to_ring
   to_nested

Interpolation
~~~~~~~~~~~~~

Interpolation from HEALPix to geographic coordinates.

.. autosummary::
   :toctree: ../generated/

   bilinear_interpolation

Hierarchy and neighbourhood
~~~~~~~~~~~~~~~~~~~~~~~~~~~

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
~~~~~~~~~~~~~~~~~~~~~

Calculate distances between HEALPix cells.

.. autosummary::
   :toctree: ../generated/

   geodesic_distances
