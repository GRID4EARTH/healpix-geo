//! Integer-only topology operations for nested cells.

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use cdshealpix::compass_point::MainWind;
use cdshealpix::nested::get;
use cdshealpix::nested::zordercurve::get_zoc;

use crate::maybe_parallelize;
use crate::vectorized::depth::Depth;

use crate::scalar::nested::topology as scalar;

/// Split cell indexes into base-face-local coordinates.
///
/// Inputs are assumed to have already been validated. The returned vectors have
/// the same length and contain `(face, i, j)` components in matching order.
pub fn healpix_to_base_cell_coordinates(
    ipix: &[u64],
    depth: Depth,
    nthreads: usize,
) -> (Vec<u8>, Vec<u32>, Vec<u32>) {
    let mut result = Vec::<(u8, u32, u32)>::with_capacity(ipix.len());

    match depth {
        Depth::Scalar(depth) => {
            maybe_parallelize!(nthreads, ipix, result, |hash| {
                scalar::healpix_to_base_cell_coordinates(*hash, *depth)
            });
        }
        Depth::Array(depths) => {
            let zipped: Vec<_> = ipix.iter().zip(depths.iter()).collect();
            maybe_parallelize!(nthreads, zipped, result, |(hash, depth)| {
                scalar::healpix_to_base_cell_coordinates(**hash, **depth)
            });
        }
    }

    let mut face = Vec::with_capacity(result.len());
    let mut x = Vec::with_capacity(result.len());
    let mut y = Vec::with_capacity(result.len());
    for (face_value, x_value, y_value) in result {
        face.push(face_value);
        x.push(x_value);
        y.push(y_value);
    }
    (face, x, y)
}

/// Combine base faces and face-local coordinates into NESTED cell indexes.
///
/// Inputs are assumed to have equal lengths and to have already been validated.
pub fn base_cell_coordinates_to_healpix(
    face: &[u8],
    x: &[u32],
    y: &[u32],
    depth: Depth,
    nthreads: usize,
) -> Vec<u64> {
    let inputs: Vec<_> = face.iter().zip(x.iter()).zip(y.iter()).collect();
    let mut result = Vec::<u64>::with_capacity(face.len());

    match depth {
        Depth::Scalar(depth) => {
            maybe_parallelize!(nthreads, inputs, result, |((face, x), y)| {
                scalar::base_cell_coordinates_to_healpix(**face, **x, **y, *depth)
            });
        }
        Depth::Array(depths) => {
            let inputs: Vec<_> = inputs.iter().zip(depths.iter()).collect();
            maybe_parallelize!(nthreads, inputs, result, |(((face, x), y), depth)| {
                scalar::base_cell_coordinates_to_healpix(**face, **x, **y, **depth)
            });
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use cdshealpix::compass_point::MainWind::{E, N, NE, NW, S, SE, SW, W};

    #[test]
    fn exhaustive_small_depth_round_trips() {
        for depth in 0..=5 {
            let npix = 12_u64 << (depth << 1);
            let pixels: Vec<_> = (0..npix).collect();
            let (face, x, y) = healpix_to_base_cell_coordinates(&pixels, Depth::Scalar(&depth), 1);
            let actual = base_cell_coordinates_to_healpix(&face, &x, &y, Depth::Scalar(&depth), 1);
            assert_eq!(actual, pixels);
        }
    }

    #[test]
    fn level_zero_is_the_base_face() {
        let pixels: Vec<_> = (0..12).collect();
        let depth = 0;
        let (face, x, y) = healpix_to_base_cell_coordinates(&pixels, Depth::Scalar(&depth), 1);
        assert_eq!(face, (0..12).collect::<Vec<_>>());
        assert_eq!(x, vec![0; 12]);
        assert_eq!(y, vec![0; 12]);
        assert_eq!(
            base_cell_coordinates_to_healpix(&face, &x, &y, Depth::Scalar(&depth), 1),
            pixels
        );
    }

    #[test]
    fn supports_per_cell_depths() {
        let depths = [0, 1, 2, 10, 29];
        let face = [0, 3, 7, 11, 5];
        let x = [0, 1, 3, 1023, (1 << 29) - 1];
        let y = [0, 0, 2, 17, 123_456_789];
        let pixels = base_cell_coordinates_to_healpix(&face, &x, &y, Depth::Array(&depths), 1);
        let actual = healpix_to_base_cell_coordinates(&pixels, Depth::Array(&depths), 1);
        assert_eq!(actual, (face.to_vec(), x.to_vec(), y.to_vec()));
    }

    #[test]
    fn face_transforms_match_cdshealpix_for_every_face_and_direction() {
        let directions = [N, NE, E, SE, S, SW, W, NW];
        let layer = get(2);

        for face in 0..12 {
            for direction in directions {
                let actual = face_neighbour_transform(face, direction);
                let expected_face = cdshealpix::neighbour(face, direction);
                assert_eq!(actual.map(|transform| transform.target_face), expected_face);

                let Some(transform) = actual else {
                    continue;
                };

                let raw: Vec<_> = (0..4)
                    .flat_map(|x| {
                        (0..4).map(move |y| {
                            layer
                                .to_neighbour_base_cell_coo(face, x, y, direction)
                                .unwrap()
                        })
                    })
                    .collect();
                let min_x = raw.iter().map(|(_, x, _)| *x).min().unwrap();
                let min_y = raw.iter().map(|(_, _, y)| *y).min().unwrap();

                for (index, (target_face, raw_x, raw_y)) in raw.into_iter().enumerate() {
                    let x = (index / 4) as i32;
                    let y = (index % 4) as i32;
                    let (mut expected_x, mut expected_y) =
                        if transform.swap_xy { (y, x) } else { (x, y) };
                    if transform.flip_x {
                        expected_x = 3 - expected_x;
                    }
                    if transform.flip_y {
                        expected_y = 3 - expected_y;
                    }

                    assert_eq!(target_face, transform.target_face);
                    assert_eq!((raw_x - min_x, raw_y - min_y), (expected_x, expected_y));
                }
            }
        }
    }
}
