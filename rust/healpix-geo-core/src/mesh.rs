//! Common functionality to convert a cell region to a mesh
//!
//! Meshes in the sense of the UGRID conventions require two things:
//!
//! - a list of deduplicated vertex coordinates
//! - indices into the vertex coordinates that form the mesh geometry
//!
//! To convert a cell region (given as a list of cell ids), we need to be able to:
//!
//! - compute global vertex ids given a cell ids
//! - compute coordinates for the global vertex ids
//! - convert vertex ids to indices
//!
//! For the vertex ids, there are a few choices:
//!
//! - ring: north pole is 0, numbering along rings of equal latitude
//! - nested: each base cell contains the southeastern and northeastern boundaries, and on
//!   conflicts the northernmost base cell wins. The poles are part of the 0th and 11th base cells.
//! - Try to use a Hilbert curve instead. For this, we somehow need to deal with the jumps in the
//!   healpix projection space.
//!
//! The functionality here requires each indexing scheme to implement a function that, given a cell id,
//! computes the vertex ids (possibly shared by converting to `(nested, depth)` or `(face, x, y, depth)` first).
//!
//! For example: vertex_hashes(hash: u64) -> CellVertices
//!
//! Other functions:
//! - vertex_indices: deduplicate the vertex ids and construct the mesh connectivity
//! - vertex coordinates: given a vertex id, compute the vertex coordinates

type CellVertices = (u64, u64, u64, u64);
type CellIndices = (usize, usize, usize, usize);

/// Deduplicate and sort the given vertex ids
pub fn vertex_indices(ipix: &[CellVertices]) -> (Vec<u64>, Vec<CellIndices>) {}

/// Convert a vertex id to coordinates
pub fn vertex_coordinates(hash: u64) -> (f64, f64) {}
