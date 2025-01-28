/// A palindromic number reads the same both ways.
/// The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 * 99.
/// Find the largest palindrome made from the product of two 3-digit numbers.
pub fn solve() -> u64 {
    let mut largest = 0;

    for a in 100..=999 {
        for b in a..=999 {
            let c = a * b;
            if c > largest && is_palindrome(c) {
                largest = c;
            }
        }
    }

    largest
}

fn is_palindrome(n: u64) -> bool {
    n == reverse(n)
}

fn reverse(mut n: u64) -> u64 {
    let mut rev = 0;
    while n > 0 {
        rev = 10 * rev + n % 10;
        n /= 10;
    }

    rev
}

check_matches!(906609);
