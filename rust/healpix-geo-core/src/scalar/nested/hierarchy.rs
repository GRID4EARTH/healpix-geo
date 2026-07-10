use cdshealpix::nested::Layer;

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
