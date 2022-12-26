mod solutions;
mod utils;

use solutions::*;
use std::env::args;
use utils::get_solution_input;

fn main() {
    let args: Vec<String> = args().collect();
    // TODO: more graceful error handling
    let day = args
        .get(1)
        .unwrap()
        .parse::<u8>()
        .expect("To convert arg to u8");
    let input = get_solution_input(day);
    let solution_fns = get_solution_fn(day);

    let solution_a = solution_fns.0(&input);
    let solution_b = solution_fns.1(&input);

    println!("Solution - part 1: {}", solution_a);
    println!("Solution - part 2: {}", solution_b);
}

// TODO: can this be automated?
fn get_solution_fn(day: u8) -> (fn(&String) -> usize, fn(&String) -> usize) {
    return match day {
        1 => (day01::solution_a, day01::solution_b),
        2 => (day02::solution_a, day02::solution_b),
        3 => (day03::solution_a, day03::solution_b),
        4 => (day04::solution_a, day04::solution_b),
        _ => panic!("Invalid day"),
    };
}
