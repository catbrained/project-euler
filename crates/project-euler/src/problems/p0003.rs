/// The prime factors of 13195 are 5, 7, 13 and 29.
/// What is the largest prime factor of the number 600851475143?
pub fn solve() -> u64 {
    let input = 600851475143;

    let factors = trial_division(input);

    factors
        .into_iter()
        .max()
        .expect("input should be either prime or a factor of primes")
}

fn trial_division(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    for d in 2..=n.isqrt() {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}

check_matches!(6857);
