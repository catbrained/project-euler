use project_euler_util::sum_of_natural_numbers;

/// The sum of the squares of the first ten natural numbers is,
/// 1^2 + 2^2 + ... + 10^2 = 385.
/// The square of the sum of the first ten natural numbers is,
/// (1 + 2 + ... + 10)^2 = 55^2 = 3025.
/// Hence the difference between the sum of the squares of the first ten natural numbers
/// and the square of the sum is 3025 - 385 = 2640.
/// Find the difference between
/// the sum of the squares of the first one hundred natural numbers and the square of the sum.
pub fn solve() -> u64 {
    let sum_of_squares = sum_of_squares(100);
    let square_of_sum = sum_of_natural_numbers(100).pow(2);

    square_of_sum - sum_of_squares
}

fn sum_of_squares(n: u64) -> u64 {
    (n * (n + 1) * (2 * n + 1)) / 6
}
