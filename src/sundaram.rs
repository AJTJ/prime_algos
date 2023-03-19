// Sieve of Sundaram
pub fn single_thread(n: usize) -> Vec<usize> {
    let k = (n - 1) / 2;
    let mut bool_vec = vec![true; k + 1];

    let sq = (k as f64).sqrt().floor() as usize;

    for i in 1..sq {
        let mut j = i;

        while i + j + 2 * i * j <= k {
            bool_vec[i + j + 2 * i * j] = false;
            j += 1
        }
    }

    let true_indices = bool_vec
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i) } else { None })
        .collect::<Vec<_>>();

    println!("sundaram, pre doubling: {true_indices:?}");

    true_indices.iter().map(|n| 2 * n + 1).collect()
}
