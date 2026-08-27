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
fn base_cell_coordinates_to_healpix(base_cell: u8, i: u32, j: u32, depth: u8) -> u64 {
    ((base_cell as u64) << (depth << 1)) | healpix::nested::zordercurve::get_zoc(depth).ij2h(i, j)
}

/// Return the canonical orientation of a neighbouring HEALPix base cell.
///
/// `None` means that the base face has no distinct neighbour in that direction.
/// The result is derived from `cdshealpix`'s coordinate transform instead of a
/// separately maintained adjacency or orientation table.
pub fn base_cell_relationship(
    base_cell: u8,
    direction: MainWind,
) -> Option<((i32, i32), (i32, i32))> {
    if base_cell >= 12 || direction == MainWind::C {
        return None;
    }

    // Any depth >= 1 is sufficient: the affine transform has the same signed
    // permutation at every depth. Sampling its basis vectors avoids duplicating
    // the canonical base_cell-orientation rules implemented by cdshealpix.
    let layer = healpix::nested::get(1);
    let (target_base_cell, origin_i, origin_j) =
        layer.to_neighbour_base_cell_coo(base_cell, 0, 0, direction)?;
    let (target_i, i_axis_i, i_axis_j) =
        layer.to_neighbour_base_cell_coo(base_cell, 1, 0, direction)?;
    let (target_j, j_axis_i, j_axis_j) =
        layer.to_neighbour_base_cell_coo(base_cell, 0, 1, direction)?;

    debug_assert_eq!(target_base_cell, target_i);
    debug_assert_eq!(target_base_cell, target_j);

    let delta_i = (i_axis_i - origin_i, i_axis_j - origin_j);
    let delta_j = (j_axis_i - origin_i, j_axis_j - origin_j);

    debug_assert_eq!(delta_i, (1 | -1, 0) | (0, 1 | -1));
    debug_assert_eq!(delta_j, (1 | -1, 0) | (0, 1 | -1));

    Some((target_base_cell, delta_i, delta_j))
}
