Module nested
==============

The module `healpix_geo.nested` gives functions for the nested indexing scheme.

.. currentmodule:: healpix_geo.nested

Coordinates Conversions
~~~~~~~~~~~~~~~~~~~~~~~~

Conversions between geographic coordinates and HEALPix indices.

.. autosummary::
   :toctree: ../generated/

   healpix_to_lonlat
   lonlat_to_healpix
   healpix_to_cartesian
   cartesian_to_healpix
   vertices

.. seealso::
   Tutorial complete : :doc:`../tutorials/coordinate_conversion`

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

   kth_neighbourhood
   zoom_to
   siblings

.. seealso::
   Complete tutorial : :doc:`../user-guide/hierarchical_indexing`

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
   Complete tutorial : :doc:`../tutorials/coverage_queries`

Distance Calculations
~~~~~~~~~~~~~~~~~~~~~

Calculate distances between HEALPix cells.

.. autosummary::
   :toctree: ../generated/

   angular_distances


Indexes and data structure
~~~~~~~~~~~~~~~~~~~~~~~~~~

Classes to manipulate HEALPix cell sets.

.. autosummary::
   :toctree: ../generated/

   RangeMOCIndex
