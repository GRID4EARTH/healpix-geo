use crate::scalar::mesh::{VertexIdScheme, encode_vertex};
use cdshealpix as healpix;

pub fn vertex_indices(depth: u8, hash: u64) -> (u64, u64, u64, u64) {
    let layer = healpix::nested::get(depth);

    let [(x_s, y_s), (x_e, y_e), (x_n, y_n), (x_w, y_w)] = layer.projected_vertices(hash);

    (
        encode_vertex(depth, x_s, y_s, VertexIdScheme::Ring),
        encode_vertex(depth, x_e, y_e, VertexIdScheme::Ring),
        encode_vertex(depth, x_n, y_n, VertexIdScheme::Ring),
        encode_vertex(depth, x_w, y_w, VertexIdScheme::Ring),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_indices() {
        assert_eq!(vertex_indices(0, 0), (5, 2, 0, 1));
        assert_eq!(vertex_indices(0, 1), (6, 3, 0, 2));

        assert_eq!(vertex_indices(1, 7), (8, 3, 0, 2));
    }
}
