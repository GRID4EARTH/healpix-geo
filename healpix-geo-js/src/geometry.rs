use cdshealpix as healpix;

const fn one_over_nside(depth: u8) -> f64 {
    let time_nside = (depth as u64) << 52;

    f64::from_bits(1_f64.to_bits() - time_nside)
}

const ONE_OVER_NSIDE: [f64; 30] = [
    one_over_nside(0),
    one_over_nside(1),
    one_over_nside(2),
    one_over_nside(3),
    one_over_nside(4),
    one_over_nside(5),
    one_over_nside(6),
    one_over_nside(7),
    one_over_nside(8),
    one_over_nside(9),
    one_over_nside(10),
    one_over_nside(11),
    one_over_nside(12),
    one_over_nside(13),
    one_over_nside(14),
    one_over_nside(15),
    one_over_nside(16),
    one_over_nside(17),
    one_over_nside(18),
    one_over_nside(19),
    one_over_nside(20),
    one_over_nside(21),
    one_over_nside(22),
    one_over_nside(23),
    one_over_nside(24),
    one_over_nside(25),
    one_over_nside(26),
    one_over_nside(27),
    one_over_nside(28),
    one_over_nside(29),
];

/// Spherical (authalic-latitude) vertex of a cell, offsets checked.
///
/// `u` and `v` are offsets from the southern vertex of the cell, and have to
/// be in `[0, 1]` (`NaN` fails the range check). The bound is also what keeps
/// `cdshealpix::unproj` from panicking — an unrecoverable wasm trap — on its
/// `-2 ≤ y ≤ 2` assertion on the projected coordinate.
pub(crate) fn spherical_vertex(
    center: (f64, f64),
    depth: u8,
    delta: (f64, f64),
) -> Result<(f64, f64), String> {
    let (u, v) = delta;
    if !(0.0..=1.0).contains(&u) || !(0.0..=1.0).contains(&v) {
        return Err(format!(
            "vertex offsets must be in [0, 1], got (u = {}, v = {})",
            u, v
        ));
    }

    let one_over_nside = ONE_OVER_NSIDE[depth as usize];

    let dx = u - v;
    let dy = u + v - 1.0;

    Ok(healpix::unproj(
        center.0 + dx * one_over_nside,
        center.1 + dy * one_over_nside,
    ))
}
