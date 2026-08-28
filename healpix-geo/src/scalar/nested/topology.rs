use cdshealpix as healpix;

#[inline]
pub fn healpix_to_base_cell_coordinates(hash: &u64, depth: &u8) -> (u8, u32, u32) {
    let depth = *depth;
    let hash = *hash;

    let twice_depth = depth << 1;
    let zoc = healpix::nested::zordercurve::get_zoc(depth);
    let ij = zoc.h2ij(hash & ((1_u64 << twice_depth) - 1));
    ((hash >> twice_depth) as u8, zoc.ij2i(ij), zoc.ij2j(ij))
}

#[inline]
pub fn base_cell_coordinates_to_healpix(base_cell: &u8, i: &u32, j: &u32, depth: &u8) -> u64 {
    ((*base_cell as u64) << (*depth << 1))
        | healpix::nested::zordercurve::get_zoc(*depth).ij2h(*i, *j)
}
