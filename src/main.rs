use prime_finder::{atkin, eratosthenes, sundaram};

fn main() {
    let erat = eratosthenes::single_thread(100);
    println!("eratosthenes: {erat:?}");

    let sund = sundaram::single_thread(100);
    println!("sundaram: {sund:?}");

    let atkin = atkin::single_thread(100);
    println!("atkin: {atkin:?}");
}

// https://www.baeldung.com/cs/prime-number-algorithms

// Sieve of Atkin
