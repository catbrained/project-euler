use project_euler_macros::gen_mods;

/// Make sure that the solution is correct.
/// Useful to make sure a refactor or optimization doesn't break things.
macro_rules! check_matches {
    ($solution:literal) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn solution_matches() {
                assert_eq!($solution, solve());
            }
        }
    };
}

gen_mods!(7);
