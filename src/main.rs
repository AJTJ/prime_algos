fn main() {
    let erat = eras_fn(100);
    println!("Erat: {:?}", erat);
}

// https://www.baeldung.com/cs/prime-number-algorithms

// Sieve of Eratosthenes
fn eras_fn(target: usize) -> Vec<usize> {
    let mut pr_ind = vec![true; target];

    let sq = (target as f64).sqrt().floor() as usize;

    for n in 2..sq {
        if pr_ind[n] == true {
            let mut j = n * n;

            while j <= target - 1 {
                pr_ind[j] = false;
                j = j + n
            }
        }
    }

    pr_ind
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b == true { Some(i) } else { None })
        .collect::<Vec<_>>()
}

// Sieve of Sundaram

// Sieve of Atkin
