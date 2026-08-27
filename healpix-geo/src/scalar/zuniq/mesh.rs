use crate::scalar::mesh::{VertexIdScheme, encode_vertex};
use cdshealpix as healpix;

pub fn vertex_indices(hash: &u64) -> (u64, u64, u64, u64) {
    let (depth, nested) = healpix::nested::from_zuniq(*hash);
    let layer = healpix::nested::get(depth);

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
        assert_eq!(vertex_indices(&288230376151711744), (5, 2, 0, 1));
        assert_eq!(vertex_indices(&864691128455135232), (6, 3, 0, 2));

        assert_eq!(vertex_indices(&1080863910568919040), (8, 3, 0, 2));
        assert_eq!(vertex_indices(&2377900603251621888), (37, 29, 21, 36));
    }
}
