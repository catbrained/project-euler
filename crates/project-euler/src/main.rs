use project_euler::problems::{solve, solve_all, PMAX};

fn main() {
    let solve_all = || {
        let results = solve_all();
        for (n, result) in results.enumerate() {
            println!("Result for problem {n}:\n\t{result}");
        }
    };

    if let Some(command) = std::env::args().nth(1) {
        match command.as_str() {
            "--all" => solve_all(),
            n if n.parse::<u16>().is_ok() => {
                let n = n.parse::<u16>().expect("Should be valid u16");
                if !(1..=PMAX).contains(&n) {
                    eprintln!("Problem must be between 1 and {PMAX}");
                } else {
                    let result = solve(n);
                    println!("Result for problem {n}:\n\t{result}");
                }
            }
            o => {
                eprintln!("Unrecognized option `{o}`. Use `--all` to run all problems, or a number between 1 and {PMAX}");
            }
        }
    } else {
        solve_all()
    }
}
