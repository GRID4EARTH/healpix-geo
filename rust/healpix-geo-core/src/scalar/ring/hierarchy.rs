use cdshealpix as healpix;

pub fn kth_neighbours(hash: &u64, nside: &u32, ring: &u32) -> Vec<i64> {
    let layer = healpix::nested::get(healpix::depth(*nside));
    let hash_nested = layer.from_ring(*hash);
    let r = *ring;

    let mut result: Vec<i64> = layer
        .kth_neighbours(hash_nested, r)
        .into_iter()
        .map(|v| v as i64)
        .map(|v| {
            if v == -1 {
                v
            } else {
                layer.to_ring(v as u64) as i64
            }
        })
        .collect();

    // 4 sides with each 2 r + 1 values, minus 4 joints: 4 * (2r + 1) - 4
    result.resize(8 * (r as usize), -1i64);

    result
}

pub fn kth_neighbourhood(hash: &u64, nside: &u32, ring: &u32) -> Vec<i64> {
    let layer = healpix::nested::get(healpix::depth(*nside));

    let hash_nested = layer.from_ring(*hash);

    let mut neighbours: Vec<i64> = layer
        .kth_neighbourhood(hash_nested, *ring)
        .into_iter()
        .map(|v| v as i64)
        .map(|v| {
            if v == -1 {
                v
            } else {
                layer.to_ring(v as u64) as i64
            }
        })
        .collect();

    let expected_size = usize::pow((2 * ring + 1) as usize, 2);
    if neighbours.len() < expected_size {
        neighbours.resize(expected_size, -1);
    }

    neighbours
}
