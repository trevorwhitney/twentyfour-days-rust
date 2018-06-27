extern crate primal;

pub fn is_prime(suspect: usize) -> bool {
    let sieve = primal::Sieve::new(10000);
    sieve.is_prime(suspect)
}

pub fn find_prime(n: usize) -> String {
    let sieve = primal::Sieve::new(10000);
    match sieve.primes_from(0).nth(n - 1) {
        Some(number) => format!("{}th prime is {}", n, number),
        None => format!("I don't know anything about {}th prime", n)
    }
}