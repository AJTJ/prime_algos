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

// pub fn multi_thread(max_target: usize) -> Vec<usize> {}
