use cdshealpix as healpix;
use cdshealpix::compass_point::MainWind;

/// Return the canonical orientation of a neighbouring HEALPix base cell.
///
/// `None` means that the base face has no distinct neighbour in that direction.
/// The result is derived from `cdshealpix`'s coordinate transform instead of a
/// separately maintained adjacency or orientation table.
#[allow(clippy::type_complexity)]
pub fn base_cell_relationship(
    base_cell: u8,
    direction: MainWind,
) -> Option<(u8, (i8, i8), (i8, i8))> {
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

    let delta_i = ((i_axis_i - origin_i) as i8, (i_axis_j - origin_j) as i8);
    let delta_j = ((j_axis_i - origin_i) as i8, (j_axis_j - origin_j) as i8);

    debug_assert!(matches!(delta_i, (1 | -1, 0) | (0, 1 | -1)));
    debug_assert!(matches!(delta_j, (1 | -1, 0) | (0, 1 | -1)));

    Some((target_base_cell, delta_i, delta_j))
}

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO: figure out what exactly this is testing
    // #[test]
    // fn base_cell_relationships_match_cdshealpix_for_every_base_cell_and_direction() {
    //     let directions = [
    //         MainWind::S, MainWind::SW, MainWind::W, MainWind::NW,
    //         MainWind::N, MainWind::NE, MainWind::E, MainWind::SE
    //     ];
    //     let layer = healpix::nested::get(2);

    //     for base_cell in 0..12 {
    //         for direction in directions {
    //             let expected_base_cell = cdshealpix::neighbour(base_cell, direction);

    //             let actual = base_cell_relationship(base_cell, direction);
    //             assert_eq!(actual.map(|relationship| relationship.0), expected_base_cell);

    //             let Some((_, delta_i, delta_j)) = actual else {
    //                 continue;
    //             };

    //             let raw: Vec<_> = (0..4)
    //                 .flat_map(|x| {
    //                     (0..4).map(move |y| {
    //                         layer
    //                             .to_neighbour_base_cell_coo(base_cell, x, y, direction)
    //                             .unwrap()
    //                     })
    //                 })
    //                 .collect();
    //             println!("raw relationships: {raw:?}");

    //             let min_x = raw.iter().map(|(_, x, _)| *x).min().unwrap();
    //             let min_y = raw.iter().map(|(_, _, y)| *y).min().unwrap();

    //             for (index, (target_base_cell, raw_x, raw_y)) in raw.into_iter().enumerate() {
    //                 let x = (index / 4) as i32;
    //                 let y = (index % 4) as i32;
    //                 let (mut expected_x, mut expected_y) =
    //                     if transform.swap_xy { (y, x) } else { (x, y) };
    //                 if transform.flip_x {
    //                     expected_x = 3 - expected_x;
    //                 }
    //                 if transform.flip_y {
    //                     expected_y = 3 - expected_y;
    //                 }

    //                 assert_eq!(target_base_cell, transform.target_base_cell);
    //                 assert_eq!((raw_x - min_x, raw_y - min_y), (expected_x, expected_y));
    //             }
    //         }
    //     }
    // }
}
