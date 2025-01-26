/// Calculates the sum of the first `n` natural numbers.
///
/// # Overflow behavior
///
/// On overflow, this function will panic if overflow checks are enabled (default in debug mode)
/// and wrap if overflow checks are disabled (default in release mode).
pub(crate) fn sum_of_natural_numbers(n: u64) -> u64 {
    (n * (n + 1)) / 2
}
