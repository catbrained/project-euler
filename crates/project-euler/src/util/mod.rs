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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_natural_numbers_matches_first_54() {
        // Numbers from https://oeis.org/A000217
        let first_54 = vec![
            0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210,
            231, 253, 276, 300, 325, 351, 378, 406, 435, 465, 496, 528, 561, 595, 630, 666, 703,
            741, 780, 820, 861, 903, 946, 990, 1035, 1081, 1128, 1176, 1225, 1275, 1326, 1378,
            1431,
        ];

        let mut res = Vec::with_capacity(54);
        for n in 0..54 {
            res.push(sum_of_natural_numbers(n));
        }

        assert_eq!(first_54, res);
    }

    #[test]
    fn fibonacci_matches_first_40_numbers() {
        // Numbers from https://oeis.org/A000045
        let first_40 = vec![
            1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
            10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269,
            2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155,
        ];

        let res: Vec<_> = fibonacci(*first_40.last().unwrap()).collect();

        assert_eq!(first_40, res);
    }

    #[test]
    fn fibonacci_finishes_within_5s() {
        let (tx, rx) = std::sync::mpsc::channel();
        let _ = std::thread::spawn(move || {
            let _: Vec<_> = fibonacci(4_000_000).collect();
            let _ = tx.send(());
        });

        assert!(rx.recv_timeout(std::time::Duration::from_secs(5)).is_ok());
    }
}
