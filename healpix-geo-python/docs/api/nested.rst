
.. module:: healpix_geo.nested

Module nested
==============

The module `healpix_geo.nested` gives functions for the nested indexing scheme.

.. currentmodule:: healpix_geo.nested

Coordinate Conversions
~~~~~~~~~~~~~~~~~~~~~~

Conversions between geographic coordinates and HEALPix indices.

.. autosummary::
   :toctree: ../generated/

   healpix_to_lonlat
   lonlat_to_healpix
   healpix_to_cartesian
   cartesian_to_healpix
   healpix_to_base_cell_coordinates
   base_cell_coordinates_to_healpix

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
   to_zuniq

Interpolation
~~~~~~~~~~~~~

Interpolation from HEALPix to geographic coordinates.

.. autosummary::
   :toctree: ../generated/

   bilinear_interpolation

Hierarchy and neighborhood
~~~~~~~~~~~~~~~~~~~~~~~~~~

Navigation in the hierarchical structure of HEALPix.

.. autosummary::
   :toctree: ../generated/

   kth_neighbours
   kth_neighbourhood
   neighbours
   zoom_to
   siblings

.. seealso::
   :doc:`tutorial </tutorials/hierarchical_indexing>` and :doc:`user-guide </user-guide/hierarchical_indexing>`

Cover Requests
~~~~~~~~~~~~~~~

Find all the cells which intersect a region.

.. autosummary::
   :toctree: ../generated/

   zone_coverage
   box_coverage
   polygon_coverage
   cone_coverage
   elliptical_cone_coverage
   internal_boundary

.. seealso::
   :doc:`tutorial </tutorials/coverage_queries>` and :doc:`user-guide </user-guide/coverage_queries>`

Distance Calculations
~~~~~~~~~~~~~~~~~~~~~

Calculate distances between HEALPix cells.

.. autosummary::
   :toctree: ../generated/

   angular_distances
   geodesic_distances


Indexes and data structure
~~~~~~~~~~~~~~~~~~~~~~~~~~

Classes to manipulate HEALPix cell sets.

.. autosummary::
   :toctree: ../generated/

   RangeMOCIndex
