enum Hands {
    Rock,
    Paper,
    Scissors,
}

enum Results {
    Win,
    Draw,
    Lose,
}

fn get_result_needed_from_input(input: &str) -> Results {
    return match input {
        "X" => Results::Lose,
        "Y" => Results::Draw,
        "Z" => Results::Win,
        _ => panic!("invalid input"),
    };
}

fn get_points_for_result(result: Results) -> u8 {
    return match result {
        Results::Win => 6,
        Results::Draw => 3,
        Results::Lose => 0,
    };
}

fn get_points_for_hand(hand: &Hands) -> u8 {
    return match hand {
        Hands::Rock => 1,
        Hands::Paper => 2,
        Hands::Scissors => 3,
    };
}

fn get_hand_from_input(input: &str) -> Hands {
    return match input {
        "A" | "X" => Hands::Rock,
        "B" | "Y" => Hands::Paper,
        "C" | "Z" => Hands::Scissors,
        _ => panic!("invalid input"),
    };
}

// TODO: Dont love this fn probs a better way
fn determine_result(opponent_hand: Hands, player_hand: &Hands) -> Results {
    match opponent_hand {
        Hands::Rock => {
            return match player_hand {
                Hands::Rock => Results::Draw,
                Hands::Paper => Results::Win,
                Hands::Scissors => Results::Lose,
            }
        }
        Hands::Paper => {
            return match player_hand {
                Hands::Rock => Results::Lose,
                Hands::Paper => Results::Draw,
                Hands::Scissors => Results::Win,
            }
        }
        Hands::Scissors => {
            return match player_hand {
                Hands::Rock => Results::Win,
                Hands::Paper => Results::Lose,
                Hands::Scissors => Results::Draw,
            }
        }
    }
}

fn get_hand_needed(opponent_hand: Hands, result: &Results) -> Hands {
    match opponent_hand {
        Hands::Rock => {
            return match result {
                Results::Win => Hands::Paper,
                Results::Draw => Hands::Rock,
                Results::Lose => Hands::Scissors,
            }
        }
        Hands::Paper => {
            return match result {
                Results::Win => Hands::Scissors,
                Results::Draw => Hands::Paper,
                Results::Lose => Hands::Rock,
            }
        }
        Hands::Scissors => {
            return match result {
                Results::Win => Hands::Rock,
                Results::Draw => Hands::Scissors,
                Results::Lose => Hands::Paper,
            }
        }
    }
}

pub fn solution_a(input: &String) -> usize {
    let mut points: usize = 0;

    for line in input.lines() {
        let (opponent_input, player_input) = line
            .split_once(" ")
            .expect("To match required input structure");

        let opponent_hand = get_hand_from_input(opponent_input);
        let player_hand = get_hand_from_input(player_input);

        let result = determine_result(opponent_hand, &player_hand);

        let round_points = get_points_for_hand(&player_hand) + get_points_for_result(result);

        points += round_points as usize;
    }

    return points;
}

pub fn solution_b(input: &String) -> usize {
    let mut points: usize = 0;

    for line in input.lines() {
        let (opponent_input, result_needed_input) = line
            .split_once(" ")
            .expect("To match required input structure");

        let opponent_hand = get_hand_from_input(opponent_input);
        let result_needed = get_result_needed_from_input(result_needed_input);

        let player_hand = get_hand_needed(opponent_hand, &result_needed);

        let round_points = get_points_for_hand(&player_hand) + get_points_for_result(result_needed);

        points += round_points as usize;
    }

    return points;
}

#[cfg(test)]
mod tests {
    use super::{solution_a, solution_b};
    use crate::utils::get_test_input;

    #[test]
    fn test_a() {
        let input = get_test_input(2);
        assert_eq!(solution_a(&input), 15);
    }

    #[test]
    fn test_b() {
        let input = get_test_input(2);
        assert_eq!(solution_b(&input), 12);
    }
}
