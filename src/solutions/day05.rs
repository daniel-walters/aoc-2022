pub fn solution_a(input: &String) -> String {
    let (stacks_input, instructions_input) = parse_input(input);
    let stacks = create_stacks(stacks_input);
    let ordered_stacks = compute_instructions_a(stacks, instructions_input);

    return get_top_crates(ordered_stacks);
}

pub fn solution_b(input: &String) -> String {
    let (stacks_input, instructions_input) = parse_input(input);
    let stacks = create_stacks(stacks_input);
    let ordered_stacks = compute_instructions_b(stacks, instructions_input);

    return get_top_crates(ordered_stacks);
}

fn get_top_crates(mut stacks: Vec<Vec<char>>) -> String {
    let mut top_crates: Vec<char> = Vec::new();
    for stack in stacks.iter_mut() {
        top_crates.push(stack.pop().unwrap());
    }

    return top_crates.iter().collect::<String>();
}

fn parse_input(input: &String) -> (&str, &str) {
    input
        .split_once("\n\n")
        .expect("to split string at empty line")
}

fn create_stacks(stack_input: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<&str> = stack_input.lines().collect();
    let num_stacks = lines
        .pop()
        .unwrap()
        .trim_end()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks as usize];

    lines.reverse();

    for line in lines {
        let line_chars = line.chars().collect::<Vec<char>>();
        let mut crates = line_chars.chunks(4);
        for stack in stacks.iter_mut() {
            let value = crates.next();

            match value {
                Some(val) => {
                    if val[0] == '[' {
                        stack.push(val[1])
                    }
                }
                None => panic!("Not enough chunks"),
            };
        }
    }

    return stacks;
}

fn get_nums_from_instructions(instruction: &str) -> Vec<u32> {
    instruction
        .split(" ")
        .filter_map(|word| word.parse().ok())
        .collect()
}

fn compute_instructions_a(mut stacks: Vec<Vec<char>>, instructions_input: &str) -> Vec<Vec<char>> {
    let lines = instructions_input.lines();
    for line in lines {
        let nums = get_nums_from_instructions(line);

        if let [times, stack_a, stack_b] = nums.as_slice() {
            for _ in 0..*times {
                let val = stacks[usize::try_from(*stack_a - 1).unwrap()].pop();
                match val {
                    Some(v) => stacks[usize::try_from(*stack_b - 1).unwrap()].push(v),
                    None => panic!("times: {}, from: {}, to: {}", times, stack_a, stack_b),
                }
            }
        }
    }

    return stacks;
}

fn compute_instructions_b(mut stacks: Vec<Vec<char>>, instructions_input: &str) -> Vec<Vec<char>> {
    let lines = instructions_input.lines();
    for line in lines {
        let nums = get_nums_from_instructions(line);

        if let [times, stack_a, stack_b] = nums.as_slice() {
            let mut temp_stack: Vec<char> = Vec::new();
            for _ in 0..*times {
                let val = stacks[usize::try_from(*stack_a - 1).unwrap()]
                    .pop()
                    .unwrap();
                temp_stack.push(val);
            }
            temp_stack.reverse();
            stacks[usize::try_from(*stack_b - 1).unwrap()].append(&mut temp_stack);
        }
    }

    return stacks;
}

#[cfg(test)]
mod tests {
    use super::{solution_a, solution_b};
    use crate::utils::get_test_input;

    #[test]
    fn test_a() {
        let input = get_test_input(5);
        assert_eq!(solution_a(&input), "CMZ");
    }

    #[test]
    fn test_b() {
        let input = get_test_input(5);
        assert_eq!(solution_b(&input), "MCD");
    }
}
