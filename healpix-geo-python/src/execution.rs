/// Run `$func` over an ndarray `Zip`, in parallel where it pays off.
///
/// Same rules as `healpix_geo::maybe_parallelize!`: `nthreads == 1` runs
/// serially, `nthreads == 0` uses rayon's global pool, and other values use a
/// cached pool from `healpix_geo::vectorized::execution::thread_pool`.
#[macro_export]
macro_rules! maybe_parallelize {
    ($nthreads:ident, $iterable:expr, $func:expr $(,)?) => {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let nthreads = $nthreads as usize;
            if nthreads == 1 {
                $iterable.for_each($func);
            } else if nthreads == 0 {
                $iterable.par_for_each($func);
            } else {
                healpix_geo::vectorized::execution::thread_pool(nthreads)
                    .install(|| $iterable.par_for_each($func));
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = &$nthreads; // no-op
            $iterable.for_each($func);
        }
    };
}
