use project_euler_util::Primes;

/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
/// What is the 10,001st prime number?
pub fn solve() -> u64 {
    Primes::with_capacity(10_001)
        // `nth` counts from 0
        .nth(10_000)
        .expect("should return a prime")
}

check_matches!(104743);
