/// Calculates the sum of the first `n` natural numbers.
///
/// # Overflow behavior
///
/// On overflow, this function will panic if overflow checks are enabled (default in debug mode)
/// and wrap if overflow checks are disabled (default in release mode).
pub(crate) fn sum_of_natural_numbers(n: u64) -> u64 {
    (n * (n + 1)) / 2
}

/// Calculates the Fibonacci sequence up to `limit`.
///
/// Note that `limit` denotes the limit for the values in the sequence.
/// It does _not_ calculate the first `limit` numbers in the sequence.
pub(crate) fn fibonacci(limit: u64) -> impl Iterator<Item = u64> {
    (0..).scan((0, 1), move |state, _elem| {
        let next = state.0 + state.1;
        let ret = state.1;
        if ret <= limit {
            *state = (state.1, next);
            Some(ret)
        } else {
            None
        }
    })
}
