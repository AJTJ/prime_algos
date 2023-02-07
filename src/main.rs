fn main() {
    let erat = eratosthenes(100);
    println!("eratosthenes: {erat:?}");

    let sund = sundaram(100);
    println!("sundaram: {sund:?}");
}

// https://www.baeldung.com/cs/prime-number-algorithms

// Sieve of Eratosthenes
fn eratosthenes(target: usize) -> Vec<usize> {
    let mut pr_ind = vec![true; target];

    let sq = (target as f64).sqrt().floor() as usize;

    for n in 2..sq {
        if pr_ind[n] {
            let mut j = n * n;

            while j < target {
                pr_ind[j] = false;
                j += n
            }
        }
    }

    pr_ind
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i) } else { None })
        .collect::<Vec<_>>()
}

// Sieve of Sundaram

fn sundaram(n: usize) -> Vec<usize> {
    let k = (n - 1) / 2;
    let mut pr_ind = vec![true; k + 1];

    let sq = (k as f64).sqrt().floor() as usize;

    for i in 1..sq {
        let mut j = i;

        while i + j + 2 * i * j <= k {
            pr_ind[i + j + 2 * i * j] = false;
            j += 1
        }
    }

    let true_indices = pr_ind
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i) } else { None })
        .collect::<Vec<_>>();

    println!("sundaram, pre doubling: {true_indices:?}");

    true_indices.iter().map(|n| 2 * n + 1).collect()
}

// Sieve of Atkin

fn atkin(n: usize) -> Vec<usize> {
    let sieve = vec![1, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53, 59];

    let mut pr_ind = vec![true; n];

    let sq = (n as f64).sqrt().floor() as usize;

    for x in 11..sq {}

    unimplemented!()
}
