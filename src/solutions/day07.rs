fn find_dirs_under_target_size(input: &String, target: usize) -> Vec<usize> {
    let mut dir_sizes: Vec<usize> = Vec::new();
    let mut candidates: Vec<usize> = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ cd") {
            let dir = line.rsplit_once(" ").unwrap().1;
            if dir == ".." {
                let size = dir_sizes.pop().unwrap();
                if size <= target {
                    candidates.push(size)
                }
            } else {
                dir_sizes.push(0);
            }
        } else if line.starts_with(|ch: char| ch.is_digit(10)) {
            let size: usize = line.split_once(" ").unwrap().0.parse().unwrap();
            for dir_size in dir_sizes.iter_mut() {
                *dir_size += size;
            }
        }
    }

    for dir_size in dir_sizes.iter() {
        if *dir_size <= target {
            candidates.push(*dir_size);
        }
    }

    return candidates;
}

pub fn solution_a(input: &String) -> usize {
    let candidates = find_dirs_under_target_size(input, 100_000);
    let total_size = candidates.iter().sum();

    return total_size;
}

pub fn solution_b(input: &String) -> usize {
    let files = find_dirs_under_target_size(input, usize::MAX);
    let root_size = files.iter().max().unwrap();
    let unused_space = 70_000_000 - root_size;
    let space_needed = 30_000_000 - unused_space;

    let file_to_delete = files
        .iter()
        .filter(|size| *size >= &space_needed)
        .min()
        .unwrap();

    return *file_to_delete;
}

#[cfg(test)]
mod tests {
    use super::{solution_a, solution_b};
    use crate::utils::get_test_input;

    #[test]
    fn test_a() {
        let input = get_test_input(7);
        assert_eq!(solution_a(&input), 95437);
    }

    #[test]
    fn test_b() {
        let input = get_test_input(7);
        assert_eq!(solution_b(&input), 24933642);
    }
}
