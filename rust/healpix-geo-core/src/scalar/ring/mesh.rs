use crate::scalar::mesh::{VertexIdScheme, encode_vertex};
use cdshealpix as healpix;

pub fn vertex_indices(depth: u8, hash: &u64) -> (u64, u64, u64, u64) {
    let layer = healpix::nested::get(depth);
    let nested = layer.from_ring(*hash);

    let [(x_s, y_s), (x_e, y_e), (x_n, y_n), (x_w, y_w)] = layer.projected_vertices(nested);

    (
        encode_vertex(depth, x_s.rem_euclid(8.0), y_s, VertexIdScheme::Ring),
        encode_vertex(depth, x_e.rem_euclid(8.0), y_e, VertexIdScheme::Ring),
        encode_vertex(depth, x_n.rem_euclid(8.0), y_n, VertexIdScheme::Ring),
        encode_vertex(depth, x_w.rem_euclid(8.0), y_w, VertexIdScheme::Ring),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_indices() {
        assert_eq!(vertex_indices(0, &0), (5, 2, 0, 1));
        assert_eq!(vertex_indices(0, &4), (9, 5, 1, 8));

        assert_eq!(vertex_indices(1, &7), (16, 9, 3, 8));
        assert_eq!(vertex_indices(1, &16), (25, 17, 9, 16));
    }
}
