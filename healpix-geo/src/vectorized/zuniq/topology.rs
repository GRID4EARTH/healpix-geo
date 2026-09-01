//! Integer-only topology operations for nested cells.

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use crate::maybe_parallelize;
use crate::vectorized::depth::Depth;

use crate::scalar::zuniq::topology as scalar;

/// Split cell indexes into base cell-local coordinates.
///
/// Inputs are assumed to have already been validated. The returned vectors have
/// the same length and contain `(base_cell, i, j)` components in matching order.
pub fn healpix_to_base_cell_coordinates(ipix: &[u64], nthreads: usize) -> Vec<(u8, u32, u32)> {
    let mut result = Vec::<(u8, u32, u32)>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, |hash| {
        scalar::healpix_to_base_cell_coordinates(hash)
    });

    result
}

/// Combine base cells and base cell-local coordinates into cell indexes.
///
/// Inputs are assumed to have equal lengths and to have already been validated.
pub fn base_cell_coordinates_to_healpix(
    coords: &[(u8, u32, u32)],
    depth: Depth,
    nthreads: usize,
) -> Vec<u64> {
    let mut result = Vec::<u64>::with_capacity(coords.len());

    match depth {
        Depth::Scalar(depth) => {
            maybe_parallelize!(nthreads, coords, result, |(base_cell, i, j)| {
                scalar::base_cell_coordinates_to_healpix(base_cell, i, j, depth)
            });
        }
        Depth::Array(depths) => {
            let zipped: Vec<_> = coords.iter().zip(depths.iter()).collect();
            maybe_parallelize!(nthreads, zipped, result, |((base_cell, i, j), depth)| {
                scalar::base_cell_coordinates_to_healpix(base_cell, i, j, depth)
            });
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use cdshealpix as healpix;

    #[test]
    fn exhaustive_small_depth_round_trips() {
        for depth in 0..=3 {
            let n_pixels = 12_u64 << (depth << 1);
            let pixels: Vec<_> = (0..n_pixels)
                .map(|nested| healpix::nested::to_zuniq(depth, nested))
                .collect();
            let coords = healpix_to_base_cell_coordinates(&pixels, 1);
            let actual = base_cell_coordinates_to_healpix(&coords, Depth::Scalar(&depth), 1);
            assert_eq!(actual, pixels);
        }
    }

    #[test]
    fn level_zero_is_the_base_base_cell() {
        let depth = 0;
        let pixels: Vec<_> = (0..12)
            .map(|nested| healpix::nested::to_zuniq(depth, nested))
            .collect();
        let actual = healpix_to_base_cell_coordinates(&pixels, 1);

        let expected: Vec<_> = (0..12)
            .zip(vec![0; 12])
            .zip(vec![0; 12])
            .map(|((base_cell, i), j)| (base_cell, i, j))
            .collect();
        assert_eq!(actual, expected);
        assert_eq!(
            base_cell_coordinates_to_healpix(&actual, Depth::Scalar(&depth), 1),
            pixels
        );
    }

    #[test]
    fn supports_per_cell_depths() {
        let depths = [0, 1, 2, 10, 29];
        let base_cell = [0, 3, 7, 11, 5];
        let i = [0, 1, 3, 1023, (1 << 29) - 1];
        let j = [0, 0, 2, 17, 123_456_789];

        let coords = base_cell
            .iter()
            .cloned()
            .zip(i.iter().cloned())
            .zip(j.iter().cloned())
            .map(|((base_cell, i), j)| (base_cell, i, j))
            .collect::<Vec<_>>();
        let pixels = base_cell_coordinates_to_healpix(&coords, Depth::Array(&depths), 1);
        let actual = healpix_to_base_cell_coordinates(&pixels, 1);
        assert_eq!(actual, coords.to_vec());
    }
}
