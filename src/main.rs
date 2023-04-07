use prime_finder::{atkin, eratosthenes, sundaram, thread_support::ThreadPool};

const TEST_NUMBER: usize = 300;

fn main() {
    let erat = eratosthenes::single_thread(TEST_NUMBER);
    println!("eratosthenes: {erat:?}");

    let sund = sundaram::single_thread(TEST_NUMBER);
    println!("sundaram: {sund:?}");

    let atkin = atkin::single_thread(TEST_NUMBER);
    println!("atkin: {atkin:?}");
}

// https://www.baeldung.com/cs/prime-number-algorithms

// Sieve of Atkin
