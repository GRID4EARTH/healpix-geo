use crate::connectivity::Connectivity;
use cdshealpix as healpix;

/// Immediate neighbours as fixed directional positions.
///
/// The order always starts at S and moves in a clock-wise direction:
/// - vertex connectivity: S, W, N, E
/// - edge connectivity: SW, NW, NE, SE
/// - all: S, SW, W, NW, N, NE, E, SE
///
/// Missing directions are represented by `-1`. Unlike `kth_neighbours`,
/// this function never compacts the remaining values when a direction is
/// missing.
pub fn neighbours(hash: &u64, connectivity: &Connectivity) -> Vec<i64> {
    let (depth, hash_nested) = healpix::nested::from_zuniq(*hash);
    let layer = healpix::nested::get(depth);
    let neighbours = layer.neighbours(hash_nested, false);

    connectivity
        .directions()
        .iter()
        .map(|direction| {
            neighbours.get(*direction).map_or(-1, |neighbour| {
                healpix::nested::to_zuniq(depth, *neighbour) as i64
            })
        })
        .collect()
}

pub fn kth_neighbours(hash: &u64, ring: &u32) -> Vec<i64> {
    let (depth, hash_nested) = healpix::nested::from_zuniq(*hash);
    let layer = healpix::nested::get(depth);
    let r = *ring;

    let mut result: Vec<i64> = layer
        .kth_neighbours(hash_nested, r)
        .into_iter()
        .map(|v| v as i64)
        .map(|v| {
            if v == -1 {
                v
            } else {
                cdshealpix::nested::to_zuniq(depth, v as u64) as i64
            }
        })
        .collect();

    // 4 sides with each 2 r + 1 values, minus 4 joints: 4 * (2r + 1) - 4
    result.resize(8 * r as usize, -1i64);

    result
}

pub fn kth_neighbourhood(hash: &u64, ring: &u32) -> Vec<i64> {
    let (depth, hash_nested) = healpix::nested::from_zuniq(*hash);
    let layer = healpix::nested::get(depth);

    let mut neighbours: Vec<i64> = layer
        .kth_neighbourhood(hash_nested, *ring)
        .into_iter()
        .map(|v| v as i64)
        .map(|v| {
            if v == -1 {
                v
            } else {
                cdshealpix::nested::to_zuniq(depth, v as u64) as i64
            }
        })
        .collect();

    let expected_size = usize::pow((2 * ring + 1) as usize, 2);
    if neighbours.len() < expected_size {
        neighbours.resize(expected_size, -1);
    }

    neighbours
}
