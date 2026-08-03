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
//! - nested: each base cell its southwestern and northwestern edges, and the western vertex.
//!   Additionally, the poles are part of the 0th and 11th base cells.
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
use crate::ellipsoid::{Ellipsoid, ReferenceBody};
use cdshealpix as healpix;
use cdshealpix::unproj;

// type CellVertices = (u64, u64, u64, u64);
// type CellIndices = (usize, usize, usize, usize);

pub(crate) enum VertexIdScheme {
    Ring,
}

#[inline]
const fn triangular_number_x4(n: u64) -> u64 {
    (n * (n + 1)) << 1
}

#[inline]
fn encode_in_ring_position(x: f64, ring: u64, nside: u64) -> u64 {
    let pole_distance = ring.min(4 * nside - ring);

    if pole_distance == 0 {
        0
    } else if pole_distance < nside {
        // original formula:
        //   i = N / 2 * ( (x - (1 - d / N) (2 floor(x/2) + 1)) mod (8d / N) )
        // floating-point optimized formula (uses only integer math)
        //   m = Nx
        //   i = ((m - (N - d) (2 (m/(2N)) + 1)) mod 4d) / 2
        let discretized_x = (nside as f64 * x) as u64;
        // integer division, the parens are significant
        let n_gaps = 2 * (discretized_x / (2 * nside)) + 1;
        let reduced_x = discretized_x - (nside - pole_distance) * n_gaps;

        reduced_x.rem_euclid(8 * pole_distance) / 2
    } else {
        // original formula
        //   p = (r + 1) mod 2 if N = 1, else r mod 2
        //   i = N / 2 ((x - p / N) mod (8 - p / N))
        // optimized formula (all integer arithmetic)
        //   i = ((N x - p) mod (8N - p)) / 2
        let phase = if nside == 1 {
            (ring + 1).rem_euclid(2)
        } else {
            ring.rem_euclid(2)
        };

        let discretized_x = (nside as f64 * x) as u64;

        (discretized_x - phase).rem_euclid(8 * nside - phase) / 2
    }
}

#[inline]
fn decode_in_ring_position(position: u64, ring: u64, nside: u64) -> f64 {
    let pole_distance = ring.min(4 * nside - ring);

    if pole_distance == 0 {
        1.0
    } else if pole_distance < nside {
        // original formula:
        //   x = 2 / N i + (1 - d / N) (2 floor(i / d) + 1)
        // optimized formula (integer arithmetic except for the last division)
        //   x = (2i + (N - d) (2 (i / d) + 1)) / N

        let gap = nside - pole_distance;
        // integer division, the parens are significant
        let completed_blocks = 2 * (position / pole_distance) + 1;

        (2 * position + gap * completed_blocks) as f64 / nside as f64
    } else {
        // original formula:
        //   x = 2/N i + p / N
        // optimized formula (division is floating point):
        //   x = (2 i + p) / N
        let phase = (if nside == 1 { ring + 1 } else { ring }).rem_euclid(2);

        (2 * position + phase) as f64 / nside as f64
    }
}

#[inline]
fn ring_offset(ring: u64, nside: u64) -> u64 {
    let pole_distance = ring.min(4 * nside - ring);

    if ring == 0 {
        0
    } else if ring < nside {
        // i_r = 1 + 2(r-1)(r-2)
        1 + triangular_number_x4(pole_distance - 1)
    } else if ring > 3 * nside {
        // i_r = 12N² + 2 - 1 - 2r(r-1)
        12 * nside.pow(2) + 1 - triangular_number_x4(pole_distance)
    } else {
        // i_r = 1 + 2(N-1)(N-2) + 4N * (r - N)
        1 + triangular_number_x4(nside - 1) + 4 * nside * (ring - nside)
    }
}

#[inline]
fn extract_ring(vertex_id: u64, nside: u64, n_vertices: u64, triangular_number: u64) -> u64 {
    if vertex_id == 0 {
        0
    } else if vertex_id == n_vertices - 1 {
        4 * nside
    } else if vertex_id < triangular_number + 1 {
        ((1 + (2 * vertex_id - 1).isqrt()) as f64 / 2.0).floor() as u64
    } else if vertex_id > n_vertices - triangular_number - 1 {
        4 * nside + 1 - (3 + (2 * (n_vertices - vertex_id) - 1).isqrt()) / 2
    } else {
        nside + (vertex_id - triangular_number - 1) / (4 * nside)
    }
}

#[inline]
pub(crate) fn encode_vertex(depth: u8, x: f64, y: f64, scheme: VertexIdScheme) -> u64 {
    let nside = healpix::nside(depth) as u64;

    match scheme {
        VertexIdScheme::Ring => {
            let ring = (nside as f64 * (2.0 - y)).floor() as u64;

            ring_offset(ring, nside) + encode_in_ring_position(x, ring, nside)
        }
    }
}

#[inline]
fn decode_vertex(depth: u8, vertex_id: u64, scheme: VertexIdScheme) -> (f64, f64) {
    let nside = healpix::nside(depth) as u64;

    match scheme {
        VertexIdScheme::Ring => {
            let n_vertices = 12 * nside * nside + 2;
            if vertex_id == 0 {
                (1.0, 2.0)
            } else if vertex_id == n_vertices - 1 {
                (1.0, -2.0)
            } else {
                let triangular_number = triangular_number_x4(nside - 1);
                let ring = extract_ring(vertex_id, nside, n_vertices, triangular_number);

                let in_ring_offset = vertex_id - ring_offset(ring, nside);
                let x = decode_in_ring_position(in_ring_offset, ring, nside);
                let y = 2.0 - ring as f64 / nside as f64;

                (x, y)
            }
        }
    }
}

// /// Deduplicate and sort the given vertex ids
// pub fn vertex_indices(ipix: &[CellVertices]) -> (Vec<u64>, Vec<CellIndices>) {}

/// Convert a vertex id to coordinates
pub fn vertex_to_geographic(depth: u8, hash: &u64, ellipsoid: &Ellipsoid) -> (f64, f64) {
    // convert vertex hash to (face, x, y)
    // - convert the vertex id into (face, x, y, depth, corner-kind)
    // - from there, convert to (x, y) healpix plane coordinates (offset from the healpix
    //   center coordinate is 1 / 2**depth)
    // - use `unproj` to compute the geographic coordinates
    let (x, y) = decode_vertex(depth, *hash, VertexIdScheme::Ring);
    if y == 2.0 {
        (0.0, 90.0)
    } else if y == -2.0 {
        (0.0, -90.0)
    } else {
        let (lon, lat) = unproj(x, y);

        (
            lon.to_degrees().rem_euclid(360.0),
            ellipsoid.latitude_authalic_to_geographic(lat).to_degrees(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ellipsoid::{ReferenceEllipsoid, ReferenceSphere};
    use assertables::assert_approx_eq;
    use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;
    use rstest::rstest;

    #[test]
    fn test_encode_in_ring_position_nside_1_polar_cap() {
        // pole
        assert_eq!(encode_in_ring_position(1.0, 0, 1), 0);
        assert_eq!(encode_in_ring_position(3.0, 0, 1), 0);
        assert_eq!(encode_in_ring_position(5.0, 0, 1), 0);
        assert_eq!(encode_in_ring_position(7.0, 0, 1), 0);

        assert_eq!(encode_in_ring_position(1.0, 4, 1), 0);
    }

    #[test]
    fn test_decode_in_ring_position_nside_1_polar_cap() {
        assert_eq!(decode_in_ring_position(0, 0, 1), 1.0);

        assert_eq!(decode_in_ring_position(0, 4, 1), 1.0);
    }

    #[test]
    fn test_encode_in_ring_position_nside_1_equatorial_region() {
        // equatorial region
        assert_eq!(encode_in_ring_position(0.0, 1, 1), 0);
        assert_eq!(encode_in_ring_position(4.0, 1, 1), 2);
        assert_eq!(encode_in_ring_position(8.0, 1, 1), 0);

        assert_eq!(encode_in_ring_position(1.0, 2, 1), 0);
        assert_eq!(encode_in_ring_position(7.0, 2, 1), 3);
    }

    #[test]
    fn test_decode_in_ring_position_nside_1_equatorial_region() {
        // equatorial region
        assert_eq!(decode_in_ring_position(0, 1, 1), 0.0);
        assert_eq!(decode_in_ring_position(2, 1, 1), 4.0);
        // assert_eq!(decode_in_ring_position(0, 1, 1), 8.0);

        assert_eq!(decode_in_ring_position(0, 2, 1), 1.0);
        assert_eq!(decode_in_ring_position(3, 2, 1), 7.0);
        assert_eq!(decode_in_ring_position(0, 3, 1), 0.0);
    }

    #[test]
    fn test_encode_in_ring_position_nside_2_polar_cap() {
        // pole
        assert_eq!(encode_in_ring_position(1.0, 0, 2), 0);
        // polar cap
        assert_eq!(encode_in_ring_position(0.5, 1, 2), 0);
        assert_eq!(encode_in_ring_position(1.5, 1, 2), 1);
        assert_eq!(encode_in_ring_position(2.5, 1, 2), 1);
        assert_eq!(encode_in_ring_position(3.5, 1, 2), 2);
        assert_eq!(encode_in_ring_position(4.5, 1, 2), 2);
        assert_eq!(encode_in_ring_position(5.5, 1, 2), 3);
        assert_eq!(encode_in_ring_position(6.5, 1, 2), 3);
        assert_eq!(encode_in_ring_position(7.5, 1, 2), 0);
    }

    #[test]
    fn test_decode_in_ring_position_nside_2_polar_cap() {
        // pole
        assert_eq!(decode_in_ring_position(0, 0, 2), 1.0);
        // polar cap
        assert_eq!(decode_in_ring_position(0, 1, 2), 0.5);
        assert_eq!(decode_in_ring_position(1, 1, 2), 2.5);
        assert_eq!(decode_in_ring_position(2, 1, 2), 4.5);
        assert_eq!(decode_in_ring_position(3, 1, 2), 6.5);
    }

    #[test]
    fn test_encode_in_ring_position_nside_2_equatorial_region() {
        // equatorial region
        assert_eq!(encode_in_ring_position(0.0, 2, 2), 0);
        assert_eq!(encode_in_ring_position(1.0, 2, 2), 1);
        assert_eq!(encode_in_ring_position(2.0, 2, 2), 2);
        assert_eq!(encode_in_ring_position(3.0, 2, 2), 3);
        assert_eq!(encode_in_ring_position(4.0, 2, 2), 4);
        assert_eq!(encode_in_ring_position(5.0, 2, 2), 5);
        assert_eq!(encode_in_ring_position(6.0, 2, 2), 6);
        assert_eq!(encode_in_ring_position(7.0, 2, 2), 7);
        assert_eq!(encode_in_ring_position(8.0, 2, 2), 0);
    }

    #[test]
    fn test_decode_in_ring_position_nside_2_equatorial_region() {
        // equatorial region
        assert_eq!(decode_in_ring_position(0, 2, 2), 0.0);
        assert_eq!(decode_in_ring_position(1, 2, 2), 1.0);
        assert_eq!(decode_in_ring_position(2, 2, 2), 2.0);
        assert_eq!(decode_in_ring_position(3, 2, 2), 3.0);
        assert_eq!(decode_in_ring_position(4, 2, 2), 4.0);
        assert_eq!(decode_in_ring_position(5, 2, 2), 5.0);
        assert_eq!(decode_in_ring_position(6, 2, 2), 6.0);
        assert_eq!(decode_in_ring_position(7, 2, 2), 7.0);
    }

    #[test]
    fn test_encode_in_ring_position_nside_4_polar_cap() {
        assert_eq!(encode_in_ring_position(0.75, 1, 4), 0);
        assert_eq!(encode_in_ring_position(1.25, 1, 4), 1);
        assert_eq!(encode_in_ring_position(2.75, 1, 4), 1);
        assert_eq!(encode_in_ring_position(3.25, 1, 4), 2);
        assert_eq!(encode_in_ring_position(4.75, 1, 4), 2);
        assert_eq!(encode_in_ring_position(5.25, 1, 4), 3);
        assert_eq!(encode_in_ring_position(6.75, 1, 4), 3);
        assert_eq!(encode_in_ring_position(7.25, 1, 4), 0);

        assert_eq!(encode_in_ring_position(0.25, 3, 4), 0);
        assert_eq!(encode_in_ring_position(0.75, 3, 4), 1);
        assert_eq!(encode_in_ring_position(1.25, 3, 4), 2);
        assert_eq!(encode_in_ring_position(1.75, 3, 4), 3);
        assert_eq!(encode_in_ring_position(2.25, 3, 4), 3);
        assert_eq!(encode_in_ring_position(2.75, 3, 4), 4);
        assert_eq!(encode_in_ring_position(3.25, 3, 4), 5);
        assert_eq!(encode_in_ring_position(3.75, 3, 4), 6);
        assert_eq!(encode_in_ring_position(4.25, 3, 4), 6);
        assert_eq!(encode_in_ring_position(4.75, 3, 4), 7);
        assert_eq!(encode_in_ring_position(5.25, 3, 4), 8);
        assert_eq!(encode_in_ring_position(5.75, 3, 4), 9);
        assert_eq!(encode_in_ring_position(6.25, 3, 4), 9);
        assert_eq!(encode_in_ring_position(6.75, 3, 4), 10);
        assert_eq!(encode_in_ring_position(7.25, 3, 4), 11);
        assert_eq!(encode_in_ring_position(7.75, 3, 4), 0);
    }

    #[test]
    fn test_decode_in_ring_position_nside_4_polar_cap() {
        assert_eq!(decode_in_ring_position(0, 1, 4), 0.75);
        assert_eq!(decode_in_ring_position(1, 1, 4), 2.75);
        assert_eq!(decode_in_ring_position(2, 1, 4), 4.75);
        assert_eq!(decode_in_ring_position(3, 1, 4), 6.75);

        assert_eq!(decode_in_ring_position(0, 2, 4), 0.5);
        assert_eq!(decode_in_ring_position(1, 2, 4), 1.0);
        assert_eq!(decode_in_ring_position(2, 2, 4), 2.5);

        assert_eq!(decode_in_ring_position(0, 3, 4), 0.25);
        assert_eq!(decode_in_ring_position(1, 3, 4), 0.75);
        assert_eq!(decode_in_ring_position(2, 3, 4), 1.25);
        // assert_eq!(decode_in_ring_position(3, 3, 4), 1.75);
        assert_eq!(decode_in_ring_position(3, 3, 4), 2.25);
        assert_eq!(decode_in_ring_position(4, 3, 4), 2.75);
        assert_eq!(decode_in_ring_position(5, 3, 4), 3.25);
        // assert_eq!(decode_in_ring_position(6, 3, 4), 3.75);
        assert_eq!(decode_in_ring_position(6, 3, 4), 4.25);
        assert_eq!(decode_in_ring_position(7, 3, 4), 4.75);
        assert_eq!(decode_in_ring_position(8, 3, 4), 5.25);
        // assert_eq!(decode_in_ring_position(9, 3, 4), 5.75);
        assert_eq!(decode_in_ring_position(9, 3, 4), 6.25);
        assert_eq!(decode_in_ring_position(10, 3, 4), 6.75);
        assert_eq!(decode_in_ring_position(11, 3, 4), 7.25);
    }

    #[test]
    fn test_encode_in_ring_position_nside_4_equatorial_region() {
        assert_eq!(encode_in_ring_position(0.0, 4, 4), 0);
        assert_eq!(encode_in_ring_position(0.5, 4, 4), 1);
        assert_eq!(encode_in_ring_position(1.0, 4, 4), 2);
        assert_eq!(encode_in_ring_position(1.5, 4, 4), 3);
        assert_eq!(encode_in_ring_position(2.0, 4, 4), 4);
        assert_eq!(encode_in_ring_position(2.5, 4, 4), 5);
        assert_eq!(encode_in_ring_position(3.0, 4, 4), 6);
        assert_eq!(encode_in_ring_position(3.5, 4, 4), 7);
        assert_eq!(encode_in_ring_position(4.0, 4, 4), 8);
        assert_eq!(encode_in_ring_position(4.5, 4, 4), 9);
        assert_eq!(encode_in_ring_position(5.0, 4, 4), 10);
        assert_eq!(encode_in_ring_position(5.5, 4, 4), 11);
        assert_eq!(encode_in_ring_position(6.0, 4, 4), 12);
        assert_eq!(encode_in_ring_position(6.5, 4, 4), 13);
        assert_eq!(encode_in_ring_position(7.0, 4, 4), 14);
        assert_eq!(encode_in_ring_position(7.5, 4, 4), 15);
        assert_eq!(encode_in_ring_position(8.0, 4, 4), 0);

        assert_eq!(encode_in_ring_position(0.25, 5, 4), 0);
        assert_eq!(encode_in_ring_position(0.75, 5, 4), 1);
        assert_eq!(encode_in_ring_position(1.25, 5, 4), 2);
        assert_eq!(encode_in_ring_position(1.75, 5, 4), 3);
        assert_eq!(encode_in_ring_position(2.25, 5, 4), 4);
        assert_eq!(encode_in_ring_position(2.75, 5, 4), 5);
        assert_eq!(encode_in_ring_position(3.25, 5, 4), 6);
        assert_eq!(encode_in_ring_position(3.75, 5, 4), 7);
        assert_eq!(encode_in_ring_position(4.25, 5, 4), 8);
        assert_eq!(encode_in_ring_position(4.75, 5, 4), 9);
        assert_eq!(encode_in_ring_position(5.25, 5, 4), 10);
        assert_eq!(encode_in_ring_position(5.75, 5, 4), 11);
        assert_eq!(encode_in_ring_position(6.25, 5, 4), 12);
        assert_eq!(encode_in_ring_position(6.75, 5, 4), 13);
        assert_eq!(encode_in_ring_position(7.25, 5, 4), 14);
        assert_eq!(encode_in_ring_position(7.75, 5, 4), 15);
    }

    #[test]
    fn test_decode_in_ring_position_nside_4_equatorial_region() {
        assert_eq!(decode_in_ring_position(0, 4, 4), 0.0);
        assert_eq!(decode_in_ring_position(1, 4, 4), 0.5);
        assert_eq!(decode_in_ring_position(2, 4, 4), 1.0);
        assert_eq!(decode_in_ring_position(3, 4, 4), 1.5);
        assert_eq!(decode_in_ring_position(4, 4, 4), 2.0);
        assert_eq!(decode_in_ring_position(5, 4, 4), 2.5);
        assert_eq!(decode_in_ring_position(6, 4, 4), 3.0);
        assert_eq!(decode_in_ring_position(7, 4, 4), 3.5);
        assert_eq!(decode_in_ring_position(8, 4, 4), 4.0);
        assert_eq!(decode_in_ring_position(9, 4, 4), 4.5);
        assert_eq!(decode_in_ring_position(10, 4, 4), 5.0);
        assert_eq!(decode_in_ring_position(11, 4, 4), 5.5);
        assert_eq!(decode_in_ring_position(12, 4, 4), 6.0);
        assert_eq!(decode_in_ring_position(13, 4, 4), 6.5);
        assert_eq!(decode_in_ring_position(14, 4, 4), 7.0);
        assert_eq!(decode_in_ring_position(15, 4, 4), 7.5);
        // assert_eq!(decode_in_ring_position(0, 4, 4), 8.0);

        assert_eq!(decode_in_ring_position(0, 5, 4), 0.25);
        assert_eq!(decode_in_ring_position(1, 5, 4), 0.75);
        assert_eq!(decode_in_ring_position(2, 5, 4), 1.25);
        assert_eq!(decode_in_ring_position(3, 5, 4), 1.75);
        assert_eq!(decode_in_ring_position(4, 5, 4), 2.25);
        assert_eq!(decode_in_ring_position(5, 5, 4), 2.75);
        assert_eq!(decode_in_ring_position(6, 5, 4), 3.25);
        assert_eq!(decode_in_ring_position(7, 5, 4), 3.75);
        assert_eq!(decode_in_ring_position(8, 5, 4), 4.25);
        assert_eq!(decode_in_ring_position(9, 5, 4), 4.75);
        assert_eq!(decode_in_ring_position(10, 5, 4), 5.25);
        assert_eq!(decode_in_ring_position(11, 5, 4), 5.75);
        assert_eq!(decode_in_ring_position(12, 5, 4), 6.25);
        assert_eq!(decode_in_ring_position(13, 5, 4), 6.75);
        assert_eq!(decode_in_ring_position(14, 5, 4), 7.25);
        assert_eq!(decode_in_ring_position(15, 5, 4), 7.75);
    }

    #[test]
    fn test_encode_in_ring_position_nside_8_polar_cap() {
        // polar cap
        assert_eq!(encode_in_ring_position(0.875, 1, 8), 0);
        assert_eq!(encode_in_ring_position(2.875, 1, 8), 1);
        assert_eq!(encode_in_ring_position(4.875, 1, 8), 2);
        assert_eq!(encode_in_ring_position(6.875, 1, 8), 3);

        assert_eq!(encode_in_ring_position(0.125, 7, 8), 0);
        assert_eq!(encode_in_ring_position(0.375, 7, 8), 1);
        assert_eq!(encode_in_ring_position(0.875, 7, 8), 3);
        assert_eq!(encode_in_ring_position(1.875, 7, 8), 7);
        assert_eq!(encode_in_ring_position(2.125, 7, 8), 7);
        assert_eq!(encode_in_ring_position(3.125, 7, 8), 11);
        assert_eq!(encode_in_ring_position(3.875, 7, 8), 14);
        assert_eq!(encode_in_ring_position(5.125, 7, 8), 18);
        assert_eq!(encode_in_ring_position(6.125, 7, 8), 21);
        assert_eq!(encode_in_ring_position(7.125, 7, 8), 25);
        assert_eq!(encode_in_ring_position(7.875, 7, 8), 0);
    }

    #[test]
    fn test_decode_in_ring_position_nside_8_polar_cap() {
        // polar cap
        assert_eq!(decode_in_ring_position(0, 1, 8), 0.875);
        assert_eq!(decode_in_ring_position(1, 1, 8), 2.875);
        assert_eq!(decode_in_ring_position(2, 1, 8), 4.875);
        assert_eq!(decode_in_ring_position(3, 1, 8), 6.875);

        assert_eq!(decode_in_ring_position(0, 7, 8), 0.125);
        assert_eq!(decode_in_ring_position(1, 7, 8), 0.375);
        assert_eq!(decode_in_ring_position(3, 7, 8), 0.875);
        // assert_eq!(decode_in_ring_position(7, 7, 8), 1.875);
        assert_eq!(decode_in_ring_position(7, 7, 8), 2.125);
        assert_eq!(decode_in_ring_position(11, 7, 8), 3.125);
        //assert_eq!(decode_in_ring_position(14, 7, 8), 3.875);
        assert_eq!(decode_in_ring_position(14, 7, 8), 4.125);
        assert_eq!(decode_in_ring_position(18, 7, 8), 5.125);
        assert_eq!(decode_in_ring_position(21, 7, 8), 6.125);
        assert_eq!(decode_in_ring_position(25, 7, 8), 7.125);
        // assert_eq!(decode_in_ring_position(0, 7, 8), 7.875);
    }

    #[test]
    fn test_encode_in_ring_position_nside_8_equatorial_region() {
        // equatorial region
        assert_eq!(encode_in_ring_position(0.00, 8, 8), 0);
        assert_eq!(encode_in_ring_position(0.25, 8, 8), 1);
        assert_eq!(encode_in_ring_position(1.00, 8, 8), 4);
        assert_eq!(encode_in_ring_position(2.00, 8, 8), 8);
        assert_eq!(encode_in_ring_position(4.00, 8, 8), 16);
        assert_eq!(encode_in_ring_position(7.75, 8, 8), 31);
        assert_eq!(encode_in_ring_position(0.125, 9, 8), 0);
        assert_eq!(encode_in_ring_position(0.375, 9, 8), 1);
        assert_eq!(encode_in_ring_position(1.125, 9, 8), 4);
        assert_eq!(encode_in_ring_position(2.125, 9, 8), 8);
        assert_eq!(encode_in_ring_position(4.125, 9, 8), 16);
        assert_eq!(encode_in_ring_position(7.875, 9, 8), 31);
    }

    #[test]
    fn test_decode_in_ring_position_nside_8_equatorial_region() {
        // equatorial region
        assert_eq!(decode_in_ring_position(0, 8, 8), 0.00);
        assert_eq!(decode_in_ring_position(1, 8, 8), 0.25);
        assert_eq!(decode_in_ring_position(4, 8, 8), 1.00);
        assert_eq!(decode_in_ring_position(8, 8, 8), 2.00);
        assert_eq!(decode_in_ring_position(16, 8, 8), 4.00);
        assert_eq!(decode_in_ring_position(31, 8, 8), 7.75);
        assert_eq!(decode_in_ring_position(0, 9, 8), 0.125);
        assert_eq!(decode_in_ring_position(1, 9, 8), 0.375);
        assert_eq!(decode_in_ring_position(4, 9, 8), 1.125);
        assert_eq!(decode_in_ring_position(8, 9, 8), 2.125);
        assert_eq!(decode_in_ring_position(16, 9, 8), 4.125);
        assert_eq!(decode_in_ring_position(31, 9, 8), 7.875);
    }

    #[test]
    fn test_ring_offsets() {
        assert_eq!(ring_offset(0, 1), 0);
        assert_eq!(ring_offset(1, 1), 1);
        assert_eq!(ring_offset(3, 1), 9);
        assert_eq!(ring_offset(4, 1), 13);

        assert_eq!(ring_offset(0, 2), 0);
        assert_eq!(ring_offset(1, 2), 1);
        assert_eq!(ring_offset(2, 2), 5);
        assert_eq!(ring_offset(5, 2), 29);
        assert_eq!(ring_offset(7, 2), 45);
        assert_eq!(ring_offset(8, 2), 49);

        assert_eq!(ring_offset(4, 4), 25);
        assert_eq!(ring_offset(12, 4), 153);

        assert_eq!(ring_offset(8, 8), 113);
        assert_eq!(ring_offset(24, 8), 625);
    }

    #[test]
    fn test_extract_ring() {
        assert_eq!(extract_ring(0, 1, 14, 0), 0);
        assert_eq!(extract_ring(1, 1, 14, 0), 1);
        assert_eq!(extract_ring(6, 1, 14, 0), 2);
        assert_eq!(extract_ring(13, 2, 50, 4), 3);
        assert_eq!(extract_ring(32, 2, 50, 4), 5);
        assert_eq!(extract_ring(21, 8, 768, 84), 3);
    }

    #[test]
    fn test_encode_vertex() {
        // north polar cap
        assert_eq!(encode_vertex(0, 0.0, 1.0, VertexIdScheme::Ring), 1);
        assert_eq!(encode_vertex(1, 0.5, 1.5, VertexIdScheme::Ring), 1);
        assert_eq!(encode_vertex(2, 2.75, 1.75, VertexIdScheme::Ring), 2);
        assert_eq!(encode_vertex(2, 1.0, 1.5, VertexIdScheme::Ring), 6);
    }

    #[test]
    fn test_decode_vertex() {
        assert_eq!(decode_vertex(0, 0, VertexIdScheme::Ring), (1.0, 2.0));
        assert_eq!(decode_vertex(0, 13, VertexIdScheme::Ring), (1.0, -2.0));
        // north polar cap
        assert_eq!(decode_vertex(0, 1, VertexIdScheme::Ring), (0.0, 1.0));
        assert_eq!(decode_vertex(1, 1, VertexIdScheme::Ring), (0.5, 1.5));
        assert_eq!(decode_vertex(1, 3, VertexIdScheme::Ring), (4.5, 1.5));
        assert_eq!(decode_vertex(2, 2, VertexIdScheme::Ring), (2.75, 1.75));
        assert_eq!(decode_vertex(2, 6, VertexIdScheme::Ring), (1.0, 1.5));

        // south polar cap
        assert_eq!(decode_vertex(0, 12, VertexIdScheme::Ring), (6.0, -1.0));
        assert_eq!(decode_vertex(1, 46, VertexIdScheme::Ring), (2.5, -1.5));

        // equator
        assert_eq!(decode_vertex(1, 5, VertexIdScheme::Ring), (0.0, 1.0));
        assert_eq!(decode_vertex(1, 7, VertexIdScheme::Ring), (2.0, 1.0));
        assert_eq!(decode_vertex(2, 25, VertexIdScheme::Ring), (0.0, 1.0));
        assert_eq!(decode_vertex(2, 32, VertexIdScheme::Ring), (3.5, 1.0));
        assert_eq!(decode_vertex(2, 41, VertexIdScheme::Ring), (0.25, 0.75));
    }

    enum TestEllipsoid {
        Sphere,
        Ellipsoid,
    }

    #[rstest]
    #[case::l0_sphere_north_pole(0, 0, TestEllipsoid::Sphere, (0.0, 90.0))]
    #[case::l0_sphere_south_pole(0, 13, TestEllipsoid::Sphere, (0.0, -90.0))]
    #[case::l0_sphere_transition_vertex(
        0, 1, TestEllipsoid::Sphere,
        (0.0, healpix::TRANSITION_LATITUDE.to_degrees()))]
    #[case::l1_sphere_equatorial1(
            1, 29, TestEllipsoid::Sphere,
            (22.5, -19.47122063449069))]
    #[case::l1_sphere_equatorial2(
        1, 36, TestEllipsoid::Sphere,
        (337.5, -19.47122063449069))]
    #[case::l1_sphere_south_polar_cap(
        1, 46, TestEllipsoid::Sphere,
        (90.0, -66.44353569089878))]
    #[case::l3_sphere_transition_vertex(
        3, 113, TestEllipsoid::Sphere,
        (0.0, healpix::TRANSITION_LATITUDE.to_degrees()))]
    #[case::l0_ellipsoid_equatorial(0, 5, TestEllipsoid::Ellipsoid, (45.0, 0.0))]
    #[case::l2_ellipsoid_equatorial(
        2, 87, TestEllipsoid::Ellipsoid,
        (326.25, 9.636338620241146))]
    #[case::l3_ellipsoid_north_polar_cap(
        3, 21, TestEllipsoid::Ellipsoid,
        (239.99999999999997, 72.46140571909436))]
    fn test_vertex_to_geographic(
        #[case] level: u8,
        #[case] vertex_id: u64,
        #[case] ellipsoid_kind: TestEllipsoid,
        #[case] expected: (f64, f64),
    ) {
        let ellipsoid = match ellipsoid_kind {
            TestEllipsoid::Sphere => Ellipsoid::Sphere(ReferenceSphere::new(
                GeodesyEllipsoid::named("sphere").unwrap(),
            )),
            TestEllipsoid::Ellipsoid => Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
                GeodesyEllipsoid::named("WGS84").unwrap(),
            )),
        };

        let actual = vertex_to_geographic(level, &vertex_id, &ellipsoid);
        assert_approx_eq!(actual.0, expected.0);
        assert_approx_eq!(actual.1, expected.1);
    }
}
