#[macro_export]
macro_rules! maybe_parallelize {
    ($nthreads:ident, $iterable:expr, $func:expr $(,)?) => {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads($nthreads as usize)
                .build()
                .unwrap();
            pool.install(|| $iterable.par_for_each($func));
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = &$nthreads; // no-op
            $iterable.for_each($func);
        }
    };
}
