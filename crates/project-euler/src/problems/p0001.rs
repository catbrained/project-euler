use crate::util::sum_of_natural_numbers;

/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
/// The sum of these multiples is 23.
/// Find the sum of all the multiples of 3 or 5 below 1000.
pub fn solve() -> u64 {
    let a = 3;
    let b = 5;
    let c = 3 * 5;
    let limit = 999;

    let sum_a = a * sum_of_natural_numbers(limit / a);
    let sum_b = b * sum_of_natural_numbers(limit / b);
    let sum_c = c * sum_of_natural_numbers(limit / c);

    sum_a + sum_b - sum_c
}

check_matches!(233168);
