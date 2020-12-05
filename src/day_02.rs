use crate::utils;

#[test]
fn part_one() {
    let mut total = 0;
    let lines = utils::file_to_lines("input/02");
    for line in lines {
        let tokens: Vec<&str> = line.split(": ").collect();
        let rule = tokens[0];
        let password = tokens[1];

        let tokens: Vec<&str> = rule.split(" ").collect();
        let limits = tokens[0];
        let character = tokens[1];

        let tokens: Vec<&str> = limits.split("-").collect();
        let lower = tokens[0].parse::<usize>().unwrap();
        let upper = tokens[1].parse::<usize>().unwrap();

        let occurance = password.matches(character).count();
        if occurance >= lower && occurance <= upper {
            total += 1;
        } else {
        }
    }
    assert_eq!(572, total);
}

#[test]
fn part_two() {
    let mut total = 0;
    let lines = utils::file_to_lines("input/02");
    for line in lines {
        let tokens: Vec<&str> = line.split(": ").collect();
        let rule = tokens[0];
        let password = tokens[1];

        let tokens: Vec<&str> = rule.split(" ").collect();
        let limits = tokens[0];
        let character = tokens[1].chars().nth(0).unwrap();

        let tokens: Vec<&str> = limits.split("-").collect();
        let lower = tokens[0].parse::<usize>().unwrap();
        let upper = tokens[1].parse::<usize>().unwrap();

        let chars: Vec<char> = password.chars().collect();

        let lower_char = chars[lower - 1];
        let upper_char = chars[upper - 1];
        let mut matches = 0;
        if lower_char == character {
            matches += 1;
        }
        if upper_char == character {
            matches += 1;
        }
        if matches == 1 {
            total += 1
        }
    }
    assert_eq!(306, total);
}
