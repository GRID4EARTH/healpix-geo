use cdshealpix as healpix;
use cdshealpix::compass_point::MainWind;

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
