use cdshealpix as healpix;
use cdshealpix::compass_point::MainWind;

#[inline]
fn healpix_to_base_cell_coordinates(hash: u64, depth: u8) -> (u8, u32, u32) {
    let twice_depth = depth << 1;
    let zoc = healpix::nested::zordercurve::get_zoc(depth);
    let ij = zoc.h2ij(hash & ((1_u64 << twice_depth) - 1));
    ((hash >> twice_depth) as u8, zoc.ij2i(ij), zoc.ij2j(ij))
}

#[inline]
fn base_cell_coordinates_to_healpix(face: u8, x: u32, y: u32, depth: u8) -> u64 {
    ((face as u64) << (depth << 1)) | healpix::nested::zordercurve::get_zoc(depth).ij2h(x, y)
}

/// Return the canonical orientation of a neighbouring HEALPix base face.
///
/// `None` means that the base face has no distinct neighbour in that direction.
/// The result is derived from `cdshealpix`'s coordinate transform instead of a
/// separately maintained adjacency or orientation table.
pub fn base_cell_relationship(face: u8, direction: MainWind) -> Option<((i32, i32), (i32, i32))> {
    if face >= 12 || direction == MainWind::C {
        return None;
    }

    // Any depth >= 1 is sufficient: the affine transform has the same signed
    // permutation at every depth. Sampling its basis vectors avoids duplicating
    // the canonical face-orientation rules implemented by cdshealpix.
    let layer = healpix::nested::get(1);
    let (target_face, origin_x, origin_y) =
        layer.to_neighbour_base_cell_coo(face, 0, 0, direction)?;
    let (target_x, x_axis_x, x_axis_y) = layer.to_neighbour_base_cell_coo(face, 1, 0, direction)?;
    let (target_y, y_axis_x, y_axis_y) = layer.to_neighbour_base_cell_coo(face, 0, 1, direction)?;
    debug_assert_eq!(target_face, target_x);
    debug_assert_eq!(target_face, target_y);

    let dx = (x_axis_x - origin_x, x_axis_y - origin_y);
    let dy = (y_axis_x - origin_x, y_axis_y - origin_y);

    debug_assert!(matches!(dx, (1 | -1, 0) | (0, 1 | -1)));
    debug_assert!(matches!(dy, (1 | -1, 0) | (0, 1 | -1)));

    Some((dx, dy))
}
