use core::time;
use std::{
    cmp,
    sync::{Arc, Mutex},
    thread,
};

use crate::thread_support::ThreadPool;

// Sieve of Eratosthenes
pub fn single_thread(max_target: usize) -> Vec<usize> {
    let mut bool_vec = vec![true; max_target];

    let sq = (max_target as f64).sqrt().floor() as usize;

    // this can't really be multi-thread since it  sequentially
    // because it would then potentially be performing extra work
    for n in 2..sq {
        if bool_vec[n] {
            let mut j = n * n;

            while j < max_target {
                bool_vec[j] = false;
                j += n
            }
        }
    }

    // this process can't really be multi-threaded since it is turning a vec of bools into
    // a vec of integers of each positive bool index.
    bool_vec
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i) } else { None })
        .collect::<Vec<_>>()
}

pub fn simple_time() {
    thread::sleep(time::Duration::from_millis(20));
}

// Sieve of Eratosthenes
pub fn multi_thread(max_target: usize) -> Vec<usize> {
    let pool = ThreadPool::new(4);

    let sq = (max_target as f64).sqrt().floor() as usize;

    let bool_vec = Arc::new(Mutex::new(vec![true; max_target]));
    let mut processed_nums: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(vec![2]));
    let mut max_processed: usize = 2;
    let mut in_process_nums: Vec<usize> = vec![];

    assert!((2..sq).len() == (sq - 2));

    // we need to loop over all the numbers and process all numbers
    // that can be processed, up to the highest value possible
    // that being said, we will need to loop again and again
    // since different values will have been completed

    // continuously loop over all possible nums
    while in_process_nums.len() < (sq - 2) {
        for n in 2..sq {
            // don't process it if it is already in_process
            if in_process_nums.contains(&n) {
                continue;
            }
            // push to in-process
            in_process_nums.push(n);

            // this should process 2, and then loop over the rest of the choices
            if n >= max_processed && n < max_processed * 2 && n < sq {
                pool.execute(|| {
                    process_val(
                        bool_vec.clone(),
                        n,
                        max_target,
                        &mut processed_nums,
                        max_processed,
                    )
                })
            }

            // the processing should be passed to a thread
            // which will then update the processed vals
        }
    }

    let bool_vec_value = bool_vec.lock().unwrap().clone();

    bool_vec_value
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i) } else { None })
        .collect::<Vec<_>>()
}

fn process_val(
    bool_vec: Arc<Mutex<Vec<bool>>>,
    n: usize,
    max_target: usize,
    processed_nums: &mut Vec<usize>,
    mut max_processed: usize,
) {
    let bool_vec_guard = bool_vec.lock().expect("mutex is poisoned");

    if bool_vec_guard[n] {
        drop(bool_vec_guard);
        let mut j = n * n;

        while j < max_target {
            let mut bool_vec_guard_two = bool_vec.lock().expect("mutex is poisoned");
            bool_vec_guard_two[j] = false;
            j += n
        }
    }
    let new_max = cmp::max(max_processed, n.clone());
    max_processed = new_max;
    processed_nums.push(n);
}
