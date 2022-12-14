use std::fs::read_to_string;

fn get_parsed_day(day: u8) -> String {
    return match day {
        1..=9 => 0.to_string() + &day.to_string(),
        10..=u8::MAX => day.to_string(),
        _ => panic!("Invalid day"),
    };
}

pub fn get_solution_input(day: u8) -> String {
    let parsed_day = get_parsed_day(day);
    let filename = format!("src/inputs/day{}/sln_input.txt", parsed_day);

    return read_input(&filename);
}

#[allow(dead_code)]
pub fn get_test_input(day: u8) -> String {
    let parsed_day = get_parsed_day(day);
    let filename = format!("src/inputs/day{}/test_input.txt", parsed_day);

    return read_input(&filename);
}

fn read_input(filename: &str) -> String {
    println!("{}", filename);
    read_to_string(filename).expect("To read file")
}
