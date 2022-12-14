pub fn solution_a(input: &String) -> usize {
    let mut sorted_calorie_list = get_calorie_totals(input);

    return match sorted_calorie_list.pop() {
        Some(num) => num,
        None => 0,
    };
}

pub fn solution_b(input: &String) -> usize {
    let mut sorted_calorie_list = get_calorie_totals(input);
    let mut accumulator: usize = 0;

    for _ in 1..=3 {
        accumulator += sorted_calorie_list.pop().unwrap_or_else(|| 0);
    }

    return accumulator;
}

fn get_calorie_totals(input: &String) -> Vec<usize> {
    let split_string = input.split("\n\n");
    let mut calories: Vec<usize> = Vec::new();

    for group in split_string {
        let mut accumulator: usize = 0;
        for line in group.lines() {
            accumulator += line.parse::<usize>().unwrap_or_else(|_| 0);
        }
        calories.push(accumulator);
    }
    calories.sort();

    return calories;
}

#[cfg(test)]
mod tests {
    use super::{solution_a, solution_b};
    use crate::utils::get_test_input;

    #[test]
    fn returns_highest_number() {
        let input = get_test_input(1);
        assert_eq!(solution_a(&input), 24000);
    }

    #[test]
    fn returns_highest_three_numbers_summed() {
        let input = get_test_input(1);
        assert_eq!(solution_b(&input), 45000);
    }
}
