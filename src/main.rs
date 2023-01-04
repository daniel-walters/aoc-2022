mod solutions;
mod utils;

use solutions::*;
use std::env::args;
use utils::get_solution_input;

trait Callable {
    fn call_function(&self, input: &String) -> String;
}

enum SolutionFunction {
    ReturnsNumber(fn(&String) -> usize),
    ReturnsString(fn(&String) -> String),
}

impl Callable for SolutionFunction {
    fn call_function(&self, input: &String) -> String {
        match self {
            SolutionFunction::ReturnsNumber(func) => return format!("{}", func(input)),
            SolutionFunction::ReturnsString(func) => return func(input),
        };
    }
}

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

    let solution_a = solution_fns.0.call_function(&input);
    let solution_b = solution_fns.1.call_function(&input);

    println!("Solution - part 1: {}", solution_a);
    println!("Solution - part 2: {}", solution_b);
}

// TODO: can this be automated?
fn get_solution_fn(day: u8) -> (SolutionFunction, SolutionFunction) {
    return match day {
        1 => (
            SolutionFunction::ReturnsNumber(day01::solution_a),
            SolutionFunction::ReturnsNumber(day01::solution_b),
        ),
        2 => (
            SolutionFunction::ReturnsNumber(day02::solution_a),
            SolutionFunction::ReturnsNumber(day02::solution_b),
        ),
        3 => (
            SolutionFunction::ReturnsNumber(day03::solution_a),
            SolutionFunction::ReturnsNumber(day03::solution_b),
        ),
        4 => (
            SolutionFunction::ReturnsNumber(day04::solution_a),
            SolutionFunction::ReturnsNumber(day04::solution_b),
        ),
        5 => (
            SolutionFunction::ReturnsString(day05::solution_a),
            SolutionFunction::ReturnsString(day05::solution_b),
        ),
        _ => panic!("Invalid day"),
    };
}
