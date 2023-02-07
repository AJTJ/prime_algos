fn main() {
    let erat = eratosthenes(100);
    println!("eratosthenes: {erat:?}");

    let sund = sundaram(100);
    println!("sundaram: {sund:?}");

    let atkin = atkin(100);
    println!("atkin: {atkin:?}");
}

// https://www.baeldung.com/cs/prime-number-algorithms

// Sieve of Eratosthenes
fn eratosthenes(target: usize) -> Vec<usize> {
    let mut bool_vec = vec![true; target];

    let sq = (target as f64).sqrt().floor() as usize;

    for n in 2..sq {
        if bool_vec[n] {
            let mut j = n * n;

            while j < target {
                bool_vec[j] = false;
                j += n
            }
        }
    }

    bool_vec
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i) } else { None })
        .collect::<Vec<_>>()
}

// Sieve of Sundaram

fn sundaram(n: usize) -> Vec<usize> {
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

// Sieve of Atkin

fn atkin(n: usize) -> Vec<usize> {
    let comparison_nums = vec![1, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53, 59];

    let mut bool_vec = vec![true; n];

    let sq = (n as f64).sqrt().floor() as usize;

    for x in 1..sq {
        for y in (1..sq).step_by(2) {
            let m = 4 * x.pow(2) + y.pow(2);
            let m_mod = m % 60;
            let nums = [1, 13, 17, 29, 37, 41, 49, 53];
            if m <= n && nums.iter().any(|e| e == &m_mod) {
                bool_vec[m] = !bool_vec[m];
            }
        }
    }

    for x in (1..sq).step_by(2) {
        for y in (2..sq).step_by(2) {
            let m = 3 * x.pow(2) + y.pow(2);
            let m_mod = m % 60;
            let nums = [7, 19, 31, 43];
            if m <= n && nums.iter().any(|e| e == &m_mod) {
                bool_vec[m] = !bool_vec[m];
            }
        }
    }

    for x in 2..sq {
        for y in ((x - 1)..1).step_by(2).rev() {
            let m = 3 * x.pow(2) + y.pow(2);
            let m_mod = m % 60;
            let nums = [11, 23, 47, 59];
            if m <= n && nums.iter().any(|e| e == &m_mod) {
                bool_vec[m] = !bool_vec[m];
            }
        }
    }

    let mut m_vec = vec![];
    for (w, s) in (0..(n / 60)).zip(comparison_nums.iter()) {
        m_vec.push(60 * w + s);
    }

    for m in m_vec.iter() {
        if m == &1 && m.pow(2) > n {
            break;
        } else {
            let mm = m.pow(2);
            if bool_vec[*m] {
                for r in m_vec.iter() {
                    let c = mm * r;
                    if c > n {
                        break;
                    } else {
                        bool_vec[c] = false;
                    }
                }
            }
        }
    }

    let true_indices = bool_vec
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i) } else { None })
        .collect::<Vec<_>>();

    let mut primes = vec![2, 3, 5];
    primes.extend(true_indices.iter().copied());

    primes
}
