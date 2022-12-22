use std::collections::HashSet;

const ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

// Can these two repeated char functions be combined?
// Optional third arg?
fn get_repeated_char_two_strings(comp_one: &str, comp_two: &str) -> char {
    let set: HashSet<char> = comp_one.chars().collect();
    let common_char = comp_two.chars().find(|ch| set.contains(ch));

    match common_char {
        Some(ch) => return ch,
        None => panic!("No duplicate characters found"),
    };
}

fn get_repeated_char_three_strings(str_one: &str, str_two: &str, str_three: &str) -> char {
    let set_one: HashSet<char> = str_one.chars().collect();
    let set_two: HashSet<char> = str_two.chars().collect();

    let common_char = str_three
        .chars()
        .find(|ch| set_one.contains(ch) && set_two.contains(ch));

    match common_char {
        Some(ch) => return ch,
        None => panic!("No duplicate characters found"),
    }
}

fn get_priority(ch: char) -> usize {
    return match ALPHABET.iter().position(|letter| &ch == letter) {
        Some(i) => i + 1,
        None => 0,
    };
}

pub fn solution_a(input: &String) -> usize {
    let mut accumulator: usize = 0;

    for line in input.lines() {
        let mid_point = line.len() / 2;
        let (comp_one, comp_two) = line.split_at(mid_point);
        let repeated_char = get_repeated_char_two_strings(comp_one, comp_two);
        let priority = get_priority(repeated_char);
        accumulator += priority;
    }

    return accumulator;
}

pub fn solution_b(input: &String) -> usize {
    let mut accumulator: usize = 0;

    let lines: Vec<&str> = input.lines().collect();
    let chunks = lines.chunks_exact(3);

    for chunk in chunks {
        let common_char = get_repeated_char_three_strings(chunk[0], chunk[1], chunk[2]);
        let priority = get_priority(common_char);
        accumulator += priority;
    }

    return accumulator;
}

#[cfg(test)]
mod tests {
    use super::{solution_a, solution_b};
    use crate::utils::get_test_input;

    #[test]
    fn test_a() {
        let input = get_test_input(3);
        assert_eq!(solution_a(&input), 157);
    }

    #[test]
    fn test_b() {
        let input = get_test_input(3);
        assert_eq!(solution_b(&input), 70);
    }
}
