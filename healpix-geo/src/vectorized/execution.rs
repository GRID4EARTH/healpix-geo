#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Arc, Mutex, OnceLock};

#[cfg(not(target_arch = "wasm32"))]
use rayon::{ThreadPool, ThreadPoolBuilder};

/// Return a thread pool with exactly `nthreads` threads.
///
/// Pools are built on first use and kept for the lifetime of the process, so
/// repeated calls with the same thread count do not spawn new OS threads.
/// `nthreads == 0` is not handled here: callers use rayon's global pool for
/// that case.
#[cfg(not(target_arch = "wasm32"))]
pub fn thread_pool(nthreads: usize) -> Arc<ThreadPool> {
    static POOLS: OnceLock<Mutex<HashMap<usize, Arc<ThreadPool>>>> = OnceLock::new();

    let mut pools = POOLS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    pools
        .entry(nthreads)
        .or_insert_with(|| {
            Arc::new(
                ThreadPoolBuilder::new()
                    .num_threads(nthreads)
                    .build()
                    .unwrap(),
            )
        })
        .clone()
}

/// Default smallest number of items handed to one thread.
///
/// Waking a pool thread costs about as much as converting a thousand cell
/// ids, so cheap per-item functions only go parallel above this size. Call
/// sites with expensive items pass a smaller `min_len`.
pub const DEFAULT_MIN_LEN: usize = 1024;

/// Map `$func` over `$iterable` into `$buffer`, in parallel where it pays off.
///
/// Work is split into chunks of at least `min_len` items (default
/// [`DEFAULT_MIN_LEN`]).
///
/// - `nthreads == 1`, or a single chunk: serial on the calling thread, no
///   thread pool.
/// - `nthreads == 0`: rayon's global pool (one thread per CPU by default, or
///   `RAYON_NUM_THREADS`).
/// - otherwise: a cached pool with `nthreads` threads, see [`thread_pool`].
///
/// Results are in input order in all cases.
#[macro_export]
macro_rules! maybe_parallelize {
    ($nthreads:ident, $iterable:expr, $buffer:ident, $func:expr, min_len = $min_len:expr $(,)?) => {{
        #[cfg(not(target_arch = "wasm32"))]
        {
            let nthreads = $nthreads as usize;
            let min_len: usize = ($min_len as usize).max(1);
            if nthreads == 1 || $iterable.len() <= min_len {
                $buffer.extend($iterable.iter().map($func));
            } else if nthreads == 0 {
                $iterable
                    .par_iter()
                    .with_min_len(min_len)
                    .map($func)
                    .collect_into_vec(&mut $buffer);
            } else {
                $crate::vectorized::execution::thread_pool(nthreads).install(|| {
                    $iterable
                        .par_iter()
                        .with_min_len(min_len)
                        .map($func)
                        .collect_into_vec(&mut $buffer)
                });
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = &$nthreads; // no-op
            $buffer.extend($iterable.iter().map($func));
        }
        $buffer.shrink_to_fit();
    }};
    ($nthreads:ident, $iterable:expr, $buffer:ident, $func:expr $(,)?) => {
        $crate::maybe_parallelize!(
            $nthreads,
            $iterable,
            $buffer,
            $func,
            min_len = $crate::vectorized::execution::DEFAULT_MIN_LEN,
        )
    };
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use std::sync::Arc;

    use rayon::prelude::*;
    use rstest::rstest;

    use super::thread_pool;

    #[test]
    fn thread_pool_is_cached() {
        let first = thread_pool(3);
        let second = thread_pool(3);

        assert!(Arc::ptr_eq(&first, &second));
        assert_eq!(first.current_num_threads(), 3);
        assert_eq!(thread_pool(2).current_num_threads(), 2);
    }

    #[rstest]
    fn maybe_parallelize_preserves_order(
        #[values(0, 1, 2, 5)] nthreads: usize,
        #[values(0, 1, 2, 1000, 5000)] size: u64,
        #[values(None, Some(1), Some(16))] min_len: Option<usize>,
    ) {
        let items: Vec<u64> = (0..size).collect();
        let mut result = Vec::<u64>::with_capacity(items.len());

        match min_len {
            None => maybe_parallelize!(nthreads, items, result, |x| x * 2),
            Some(min_len) => {
                maybe_parallelize!(nthreads, items, result, |x| x * 2, min_len = min_len)
            }
        }

        let expected: Vec<u64> = (0..size).map(|x| x * 2).collect();
        assert_eq!(result, expected);
    }
}
