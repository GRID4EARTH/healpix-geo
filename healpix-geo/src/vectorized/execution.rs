#[macro_export]
macro_rules! maybe_parallelize {
    ($nthreads:ident, $iterable:expr, $buffer:ident, $func:expr $(,)?) => {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads($nthreads as usize)
                .build()
                .unwrap();
            pool.install(|| {
                $iterable
                    .par_iter()
                    .map($func)
                    .collect_into_vec(&mut $buffer)
            });
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = &$nthreads; // no-op
            $buffer.extend($iterable.iter().map($func));
        }
        $buffer.shrink_to_fit();
    };
}
