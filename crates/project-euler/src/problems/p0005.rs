/// 2520 is the smallest number that can be divided by
/// each of the numbers from 1 to 10 without any remainder.
/// What is the smallest positive number that is
/// evenly divisible (divisible with no remainder) by
/// all of the numbers from 1 to 20?
pub fn solve() -> u64 {
    // We are searching for the least common multiple of the numbers from 1 to 20:
    //
    // lcm(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20)
    //
    // We factor the numbers into their prime components:
    //
    // 2 -> 2, 3 -> 3, 4 -> 2^2, 5 -> 5, 6 -> 2 * 3, 7 -> 7, 8 -> 2^3, 9 -> 3^2,
    // 10 -> 2 * 5, 11 -> 11, 12 -> 2^2 * 3, 13 -> 13, 14 -> 2 * 7, 15 -> 3 * 5,
    // 16 -> 2^4, 17 -> 17, 18 -> 2 * 3^2, 19 -> 19, 20 -> 2^2 * 5
    //
    // Then we multiply all the highest powers of each prime:
    2_u64.pow(4) * 3_u64.pow(2) * 5 * 7 * 11 * 13 * 17 * 19
}

check_matches!(232792560);
