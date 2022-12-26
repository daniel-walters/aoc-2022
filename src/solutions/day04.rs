type SectionType = (u8, u8);

struct SectionPairs {
    elf_a: SectionType,
    elf_b: SectionType,
}

impl SectionPairs {
    fn new(a: SectionType, b: SectionType) -> SectionPairs {
        SectionPairs { elf_a: a, elf_b: b }
    }

    fn contains(&self) -> bool {
        let SectionPairs { elf_a, elf_b } = self;
        // better way to do this?
        (elf_b.0 >= elf_a.0 && elf_b.1 <= elf_a.1) || (elf_a.0 >= elf_b.0 && elf_a.1 <= elf_b.1)
    }

    fn overlap(&self) -> bool {
        let SectionPairs { elf_a, elf_b } = self;
        // better way to do this?
        elf_a.0 <= elf_b.1 && elf_b.0 <= elf_a.1
    }
}

fn input_to_section_pairs(input: &str) -> SectionPairs {
    let (elf_a, elf_b) = input.split_once(",").unwrap();

    let a_sections = elf_a.split_once("-").unwrap();
    let b_sections = elf_b.split_once("-").unwrap();

    return SectionPairs::new(
        (a_sections.0.parse().unwrap(), a_sections.1.parse().unwrap()),
        (b_sections.0.parse().unwrap(), b_sections.1.parse().unwrap()),
    );
}

pub fn solution_a(input: &String) -> usize {
    let mut accumulator: usize = 0;
    for line in input.lines() {
        let section_pairs = input_to_section_pairs(line);
        let sections_contain_eacother = section_pairs.contains();
        if sections_contain_eacother {
            accumulator += 1
        };
    }

    return accumulator;
}

pub fn solution_b(input: &String) -> usize {
    let mut accumulator: usize = 0;
    for line in input.lines() {
        let section_pairs = input_to_section_pairs(line);
        let sections_overlap = section_pairs.overlap();
        if sections_overlap {
            accumulator += 1
        };
    }

    return accumulator;
}

#[cfg(test)]
mod tests {
    use super::{solution_a, solution_b};
    use crate::utils::get_test_input;

    #[test]
    fn test_a() {
        let input = get_test_input(4);
        assert_eq!(solution_a(&input), 2);
    }

    #[test]
    fn test_b() {
        let input = get_test_input(4);
        assert_eq!(solution_b(&input), 4);
    }
}
