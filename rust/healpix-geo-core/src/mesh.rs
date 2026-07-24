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
use cdshealpix as healpix;
// use cdshealpix::unproj;

// type CellVertices = (u64, u64, u64, u64);
// type CellIndices = (usize, usize, usize, usize);

enum VertexIdScheme {
    Ring,
    ZOrder,
}

#[inline]
const fn triangular_number_x4(n: u64) -> u64 {
    (n * (n + 1)) << 1
}

#[inline]
const fn polar_cap_vertices(nside: u32) -> u64 {
    triangular_number_x4(nside as u64 - 1) + 1
}

#[inline]
fn ring(y: f64, nside: u32) -> u64 {
    ((y + 2.0) * nside as f64).floor() as u64
}

#[inline]
fn first_node_in_ring(ring: u64, nside: u64) -> f64 {
    if ring < nside || (nside >= 2 * nside && nside < 3 * nside) {
        1.0 - ring.rem_euclid(nside) as f64 / nside as f64
    } else {
        ring.rem_euclid(nside) as f64 / nside as f64
    }
}

#[inline]
fn in_ring_position(x: f64, ring: u64, nside: u64) -> u64 {
    let pole_distance = ring.min(4 * nside - ring);

    if pole_distance == 0 {
        0
    } else if pole_distance < nside {
        let occupied = pole_distance as f64 / nside as f64;
        let reduced_x = x - (1.0 - occupied) * (2.0 * (x / 2.0).floor() + 1.0);
        let collapsed_size = 8.0 * pole_distance as f64 / nside as f64;

        (nside as f64 / 2.0 * reduced_x.rem_euclid(collapsed_size)).floor() as u64
    } else {
        let phase = if nside == 1 { 0 } else { ring.rem_euclid(2) };
        let reduced_x = x - phase as f64 / nside as f64;

        let collapsed_ring_size = 8.0 - phase as f64 / nside as f64;
        (nside as f64 / 2.0 * reduced_x.rem_euclid(collapsed_ring_size)).floor() as u64
    }
}

#[inline]
const fn equatorial_vertices(nside: u32) -> u64 {
    let nside = nside as u64;
    let rings = 2 * nside + 1;

    rings * 4 * nside
}

#[inline]
fn ring_offset(ring: u64, nside: u64) -> u64 {
    let pole_distance = ring.min(4 * nside - ring);

    if ring == 0 {
        0
    } else if ring < nside {
        1 + 2 * pole_distance * (pole_distance - 1)
    } else if ring > 3 * nside {
        12 * nside.pow(2) + 1 - 2 * (pole_distance + 1) * pole_distance
    } else {
        1 + 2 * nside * (nside - 1) + 4 * nside * (ring - nside)
    }
}

fn encode_vertex(depth: u8, x: f64, y: f64, scheme: VertexIdScheme) -> u64 {
    let nside = healpix::nside(depth) as u64;

    match scheme {
        VertexIdScheme::Ring => {
            let ring = (nside as f64 * (2.0 - y)).floor() as u64;

            println!(
                "ring: {ring}, ({} + {}) ({nside})",
                ring_offset(ring, nside),
                in_ring_position(x, ring, nside)
            );

            ring_offset(ring, nside) + in_ring_position(x, ring, nside)
        }
        VertexIdScheme::ZOrder => todo!(),
    }
}

// fn decode_vertex(vertex_id: u64, scheme: VertexIdScheme) -> (f64, f64) {}

// /// Deduplicate and sort the given vertex ids
// pub fn vertex_indices(ipix: &[CellVertices]) -> (Vec<u64>, Vec<CellIndices>) {}

// /// Convert a vertex id to coordinates
// pub fn vertex_coordinates(hash: u64) -> (f64, f64) {
//     // convert vertex hash to (face, x, y)
//     // - convert the vertex id into (face, x, y, depth, corner-kind)
//     // - from there, convert to (x, y) healpix plane coordinates (offset from the healpix
//     //   center coordinate is 1 / 2**depth)
//     // - use `unproj` to compute the geographic coordinates
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polar_cap_vertices() {
        let actual = polar_cap_vertices(1);
        assert_eq!(actual, 1);

        let actual = polar_cap_vertices(2);
        assert_eq!(actual, 5);

        let actual = polar_cap_vertices(4);
        assert_eq!(actual, 25);
    }

    #[test]
    fn test_equatorial_vertices() {
        let actual = equatorial_vertices(1);
        assert_eq!(actual, 12);

        let actual = equatorial_vertices(2);
        assert_eq!(actual, 40);

        let actual = equatorial_vertices(4);
        assert_eq!(actual, 144);
    }

    #[test]
    fn test_in_ring_position_nside_1_polar_caps() {
        // pole
        assert_eq!(in_ring_position(1.0, 0, 1), 0);
        assert_eq!(in_ring_position(3.0, 0, 1), 0);
        assert_eq!(in_ring_position(5.0, 0, 1), 0);
        assert_eq!(in_ring_position(7.0, 0, 1), 0);

        assert_eq!(in_ring_position(1.0, 4, 1), 0);
    }

    #[test]
    fn test_in_ring_position_nside_1_equatorial_region() {
        // equatorial region
        assert_eq!(in_ring_position(0.0, 1, 1), 0);
        assert_eq!(in_ring_position(4.0, 1, 1), 2);
        assert_eq!(in_ring_position(8.0, 1, 1), 0);

        assert_eq!(in_ring_position(1.0, 2, 1), 0);
        assert_eq!(in_ring_position(7.0, 2, 1), 3);
    }

    #[test]
    fn test_in_ring_position_nside_2_polar_cap() {
        // pole
        assert_eq!(in_ring_position(1.0, 0, 2), 0);
        // polar cap
        assert_eq!(in_ring_position(0.5, 1, 2), 0);
        assert_eq!(in_ring_position(1.5, 1, 2), 1);
        assert_eq!(in_ring_position(2.5, 1, 2), 1);
        assert_eq!(in_ring_position(3.5, 1, 2), 2);
        assert_eq!(in_ring_position(4.5, 1, 2), 2);
        assert_eq!(in_ring_position(5.5, 1, 2), 3);
        assert_eq!(in_ring_position(6.5, 1, 2), 3);
        assert_eq!(in_ring_position(7.5, 1, 2), 0);
    }

    #[test]
    fn test_in_ring_position_nside_2_equatorial_region() {
        // equatorial region
        assert_eq!(in_ring_position(0.0, 2, 2), 0);
        assert_eq!(in_ring_position(1.0, 2, 2), 1);
        assert_eq!(in_ring_position(2.0, 2, 2), 2);
        assert_eq!(in_ring_position(3.0, 2, 2), 3);
        assert_eq!(in_ring_position(4.0, 2, 2), 4);
        assert_eq!(in_ring_position(5.0, 2, 2), 5);
        assert_eq!(in_ring_position(6.0, 2, 2), 6);
        assert_eq!(in_ring_position(7.0, 2, 2), 7);
        assert_eq!(in_ring_position(8.0, 2, 2), 0);
    }

    #[test]
    fn test_in_ring_position_nside_4_polar_cap() {
        assert_eq!(in_ring_position(0.75, 1, 4), 0);
        assert_eq!(in_ring_position(1.25, 1, 4), 1);
        assert_eq!(in_ring_position(2.75, 1, 4), 1);
        assert_eq!(in_ring_position(3.25, 1, 4), 2);
        assert_eq!(in_ring_position(4.75, 1, 4), 2);
        assert_eq!(in_ring_position(5.25, 1, 4), 3);
        assert_eq!(in_ring_position(6.75, 1, 4), 3);
        assert_eq!(in_ring_position(7.25, 1, 4), 0);

        assert_eq!(in_ring_position(0.25, 3, 4), 0);
        assert_eq!(in_ring_position(0.75, 3, 4), 1);
        assert_eq!(in_ring_position(1.25, 3, 4), 2);
        assert_eq!(in_ring_position(1.75, 3, 4), 3);
        assert_eq!(in_ring_position(2.25, 3, 4), 3);
        assert_eq!(in_ring_position(2.75, 3, 4), 4);
        assert_eq!(in_ring_position(3.25, 3, 4), 5);
        assert_eq!(in_ring_position(3.75, 3, 4), 6);
        assert_eq!(in_ring_position(4.25, 3, 4), 6);
        assert_eq!(in_ring_position(4.75, 3, 4), 7);
        assert_eq!(in_ring_position(5.25, 3, 4), 8);
        assert_eq!(in_ring_position(5.75, 3, 4), 9);
        assert_eq!(in_ring_position(6.25, 3, 4), 9);
        assert_eq!(in_ring_position(6.75, 3, 4), 10);
        assert_eq!(in_ring_position(7.25, 3, 4), 11);
        assert_eq!(in_ring_position(7.75, 3, 4), 0);
    }

    #[test]
    fn test_in_ring_position_nside_4_equatorial_region() {
        assert_eq!(in_ring_position(0.0, 4, 4), 0);
        assert_eq!(in_ring_position(0.5, 4, 4), 1);
        assert_eq!(in_ring_position(1.0, 4, 4), 2);
        assert_eq!(in_ring_position(1.5, 4, 4), 3);
        assert_eq!(in_ring_position(2.0, 4, 4), 4);
        assert_eq!(in_ring_position(2.5, 4, 4), 5);
        assert_eq!(in_ring_position(3.0, 4, 4), 6);
        assert_eq!(in_ring_position(3.5, 4, 4), 7);
        assert_eq!(in_ring_position(4.0, 4, 4), 8);
        assert_eq!(in_ring_position(4.5, 4, 4), 9);
        assert_eq!(in_ring_position(5.0, 4, 4), 10);
        assert_eq!(in_ring_position(5.5, 4, 4), 11);
        assert_eq!(in_ring_position(6.0, 4, 4), 12);
        assert_eq!(in_ring_position(6.5, 4, 4), 13);
        assert_eq!(in_ring_position(7.0, 4, 4), 14);
        assert_eq!(in_ring_position(7.5, 4, 4), 15);
        assert_eq!(in_ring_position(8.0, 4, 4), 0);

        assert_eq!(in_ring_position(0.25, 5, 4), 0);
        assert_eq!(in_ring_position(0.75, 5, 4), 1);
        assert_eq!(in_ring_position(1.25, 5, 4), 2);
        assert_eq!(in_ring_position(1.75, 5, 4), 3);
        assert_eq!(in_ring_position(2.25, 5, 4), 4);
        assert_eq!(in_ring_position(2.75, 5, 4), 5);
        assert_eq!(in_ring_position(3.25, 5, 4), 6);
        assert_eq!(in_ring_position(3.75, 5, 4), 7);
        assert_eq!(in_ring_position(4.25, 5, 4), 8);
        assert_eq!(in_ring_position(4.75, 5, 4), 9);
        assert_eq!(in_ring_position(5.25, 5, 4), 10);
        assert_eq!(in_ring_position(5.75, 5, 4), 11);
        assert_eq!(in_ring_position(6.25, 5, 4), 12);
        assert_eq!(in_ring_position(6.75, 5, 4), 13);
        assert_eq!(in_ring_position(7.25, 5, 4), 14);
        assert_eq!(in_ring_position(7.75, 5, 4), 15);
    }

    #[test]
    fn test_in_ring_position_nside_8_polar_caps() {
        // polar cap
        assert_eq!(in_ring_position(0.875, 1, 8), 0);
        assert_eq!(in_ring_position(2.875, 1, 8), 1);
        assert_eq!(in_ring_position(4.875, 1, 8), 2);
        assert_eq!(in_ring_position(6.875, 1, 8), 3);

        assert_eq!(in_ring_position(0.125, 7, 8), 0);
        assert_eq!(in_ring_position(0.375, 7, 8), 1);
        assert_eq!(in_ring_position(0.875, 7, 8), 3);
        assert_eq!(in_ring_position(1.875, 7, 8), 7);
        assert_eq!(in_ring_position(2.125, 7, 8), 7);
        assert_eq!(in_ring_position(3.125, 7, 8), 11);
        assert_eq!(in_ring_position(3.875, 7, 8), 14);
        assert_eq!(in_ring_position(5.125, 7, 8), 18);
        assert_eq!(in_ring_position(6.125, 7, 8), 21);
        assert_eq!(in_ring_position(7.125, 7, 8), 25);
        assert_eq!(in_ring_position(7.875, 7, 8), 0);
    }

    #[test]
    fn test_in_ring_position_nside_8_equatorial_region() {
        // equatorial region
        assert_eq!(in_ring_position(0.00, 8, 8), 0);
        assert_eq!(in_ring_position(0.25, 8, 8), 1);
        assert_eq!(in_ring_position(1.00, 8, 8), 4);
        assert_eq!(in_ring_position(2.00, 8, 8), 8);
        assert_eq!(in_ring_position(4.00, 8, 8), 16);
        assert_eq!(in_ring_position(7.75, 8, 8), 31);
        assert_eq!(in_ring_position(0.125, 9, 8), 0);
        assert_eq!(in_ring_position(0.375, 9, 8), 1);
        assert_eq!(in_ring_position(1.125, 9, 8), 4);
        assert_eq!(in_ring_position(2.125, 9, 8), 8);
        assert_eq!(in_ring_position(4.125, 9, 8), 16);
        assert_eq!(in_ring_position(7.875, 9, 8), 31);
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
    fn test_encode_vertex() {
        // north polar cap
        assert_eq!(encode_vertex(0, 0.0, 1.0, VertexIdScheme::Ring), 1);
        assert_eq!(encode_vertex(1, 0.5, 1.5, VertexIdScheme::Ring), 1);
        assert_eq!(encode_vertex(2, 2.75, 1.75, VertexIdScheme::Ring), 2);
        assert_eq!(encode_vertex(2, 1.0, 1.5, VertexIdScheme::Ring), 6);
    }
}
