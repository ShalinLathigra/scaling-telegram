use std::{env, error::Error};

pub mod context;
pub mod daysix;
pub mod dayseven;
pub mod dayeight;

const SOLVERS: [for<'a> fn(&'a context::Context) -> Option<bool>; 6] = [daysix::solve_a, daysix::solve_b, dayseven::solve_a, dayseven::solve_b, dayeight::solve_a, dayeight::solve_b];

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    // parse usage
    // expected input is: <run> day a/b test/real
    if args.len() < 3 {
        eprintln!("Expected usage: <run> day# a/b test/real(optional)");
        return Ok(());
    }
    let (day, variant, mut is_test) = (args[1].as_str(), args[2].as_str(), false);

    if args.len() >= 4 {
        is_test = args[3] == "test";
    }

    // instantiate test runner
    let context = &context::Context::from(day, is_test)?;

    let mut solver_index = ((day.parse::<u32>().unwrap() - 6) * 2) as usize;
    match variant {
        "b" => solver_index += 1,
        _ => ()
    };

    if solver_index > SOLVERS.len() {
        eprintln!("Error parsing combination: {} {}", day, variant);
        return Ok(());
    }
    match SOLVERS[solver_index](context) {
        Some(true) => println!("Pass! :vD"),
        _ => println!("Fail! :v<")
    }
    Ok(())
}

/*

Define modules for each AOC challenge, store inputs all in the inputs folder

*/