use cdshealpix as healpix;

pub fn to_zuniq(hash: &u64, depth: &u8) -> u64 {
    healpix::nested::to_zuniq_unsafe(*depth, *hash)
}

pub fn to_ring(hash: &u64, depth: &u8) -> u64 {
    healpix::nested::get(*depth).to_ring(*hash)
}
