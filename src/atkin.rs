// I don't think this necessarily works.

pub fn single_thread(n: usize) -> Vec<usize> {
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

    let w_range = 0..(n / 60);

    println!("w_range: {w_range:?}");

    let mut m_vec = vec![];
    for (w, s) in w_range.zip(comparison_nums.iter()) {
        println!("W: {w}, S: {s}");
        m_vec.push(60 * w + s);
    }

    for m in m_vec.iter() {
        println!("M: {m}, POW: {}", m.pow(2));
        if m.pow(2) > n {
            break;
        } else if m != &1 {
            let mm = m.pow(2);
            if bool_vec[*m] {
                for r in m_vec.iter() {
                    let c = mm * r;
                    print!("C: {c}");
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
    primes.extend(true_indices.iter());

    primes
}
