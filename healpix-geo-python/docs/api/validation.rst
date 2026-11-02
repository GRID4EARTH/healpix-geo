Module validation
=================

The module `healpix_geo.validation` gives functions to validate inputs.

``check_`` functions only verify the input values are in their valid range and
raise exceptions if not. ``validate_`` functions apply the same check and return
the input as a numpy array of the correct dtype that can be passed to the Rust
interface.

.. currentmodule:: healpix_geo.validation

Depth
~~~~~

If depth is an integer, use :func:`check_depth` to verify the depth is in
:math:`[0, 29]`. If the depth can be array-like, use :func:`validate_depth` to
check the input and obtain an int or an array of ``np.uint8`` (depending on
the input type).

.. autosummary::
   :toctree: ../generated/

    check_depth
    validate_depth

Ring
~~~~

For neighbourhood functions, use :func:`check_ring` to verify the ring argument
is in the range :math:`[0, 2^{depth}]`.

.. autosummary::
   :toctree: ../generated/

    check_ring

HEALPix cell indices
~~~~~~~~~~~~~~~~~~~~

Use :func:`validate_cell_ids` to check the input values and obtain an array of
``np.uint64``. It checks the indices are in the range
:math:`[0, 12\cdot 4^{depth}[`. If the depth is omitted, it only checks the
indices are positive (for zuniq).

.. autosummary::
   :toctree: ../generated/

    check_cell_ids
    validate_cell_ids

Geographic coordinates
~~~~~~~~~~~~~~~~~~~~~~

For floats, there is no specific checks. Simply use
``np.atleast_1d(x).astype(np.float64)``. Coordinates typically require multiple
array inputs that should be of the same shape. Use :func:`check_same_shape` to
check.

.. autosummary::
   :toctree: ../generated/

    check_same_shape

Tuples
~~~~~~

Coverage functions take in tuples or 1D array of a specific length.
Use :func:`validate_tuple2` and :func:`validate_tuple4` to obtain a tuple of the
corresponding length.

.. autosummary::
   :toctree: ../generated/

    validate_tuple2
    validate_tuple4
