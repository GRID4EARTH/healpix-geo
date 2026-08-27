use cdshealpix as healpix;

pub fn to_zuniq(hash: &u64, depth: &u8) -> u64 {
    let hash_nested = healpix::nested::get(*depth).from_ring(*hash);

    healpix::nested::to_zuniq_unsafe(*depth, hash_nested)
}

pub fn to_nested(hash: &u64, depth: &u8) -> u64 {
    healpix::nested::get(*depth).from_ring(*hash)
}
