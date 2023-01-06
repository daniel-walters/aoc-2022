use std::collections::HashSet;

pub fn solution_a(input: &String) -> usize {
    find_marker(input, 4)
}

pub fn solution_b(input: &String) -> usize {
    find_marker(input, 14)
}

fn find_marker(input: &String, window_size: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let char_windows = chars.windows(window_size);

    let mut index: usize = window_size;
    let mut found = false;

    for win in char_windows {
        let mut duplicate_set: HashSet<&char> = HashSet::new();

        for (i, ch) in win.iter().enumerate() {
            match duplicate_set.contains(ch) {
                true => {
                    break;
                }
                false => {
                    duplicate_set.insert(ch);
                    if i == window_size - 1 {
                        found = true;
                    }
                }
            }
        }

        match found {
            true => {
                return index;
            }
            false => {
                index += 1;
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::{solution_a, solution_b};
    use crate::utils::get_test_input;

    #[test]
    fn test_a() {
        let input = get_test_input(6);
        assert_eq!(solution_a(&input), 7);
    }

    #[test]
    fn test_b() {
        let input = get_test_input(6);
        assert_eq!(solution_b(&input), 19);
    }
}
