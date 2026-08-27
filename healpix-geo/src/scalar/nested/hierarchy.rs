use crate::connectivity::Connectivity;
use cdshealpix::nested::Layer;

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
pub fn neighbours(hash: &u64, layer: &Layer, connectivity: &Connectivity) -> Vec<i64> {
    let neighbours = layer.neighbours(*hash, false);

    connectivity
        .directions()
        .iter()
        .map(|direction| {
            neighbours
                .get(*direction)
                .map_or(-1, |neighbour| *neighbour as i64)
        })
        .collect()
}

pub fn kth_neighbours(hash: &u64, layer: &Layer, ring: &u32) -> Vec<i64> {
    let r = *ring;
    let mut result = layer
        .kth_neighbours(*hash, r)
        .into_iter()
        .map(|v| v as i64)
        .collect::<Vec<i64>>();

    // 4 sides with each 2 r + 1 values, minus 4 joints: 4 * (2r + 1) - 4 = 8r
    result.resize(8 * r as usize, -1i64);

    result
}

pub fn kth_neighbourhood(hash: &u64, layer: &Layer, ring: &u32) -> Vec<i64> {
    let mut neighbours: Vec<i64> = layer
        .kth_neighbourhood(*hash, *ring)
        .into_iter()
        .map(|v| v as i64)
        .collect();

    let expected_size = usize::pow((2 * ring + 1) as usize, 2);
    if neighbours.len() < expected_size {
        neighbours.resize(expected_size, -1);
    }

    neighbours
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_neighbours() {
        let layer = cdshealpix::nested::get(2);

        let result = neighbours(&42, layer, &Connectivity::Edge);

        assert_eq!(result, [111, 21, 43, 40]);
    }

    #[test]
    fn test_all_neighbours_preserves_missing_direction() {
        let layer = cdshealpix::nested::get(2);

        let result = neighbours(&42, layer, &Connectivity::All);

        assert_eq!(result, [109, 111, -1, 21, 23, 43, 41, 40]);
    }
}
