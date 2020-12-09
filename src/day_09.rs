#[allow(unused_imports)]
use crate::utils;

#[test]
fn part_one() {
    const PRELUDE: usize = 25;
    let lines = utils::file_to_lines("input/09");
    let numbers: Vec<i128> = lines
        .into_iter()
        .map(|x| x.parse::<i128>().unwrap())
        .collect();

    for x in PRELUDE..numbers.len() {
        let mut is_valid = false;

        for y in x - PRELUDE..x {
            for z in x - PRELUDE..x {
                if numbers[x] == numbers[y] + numbers[z] {
                    is_valid = true;
                }
            }
        }
        if !is_valid {
            println!("DAY 09 : PART 1 : {}", numbers[x]);
            assert_eq!(257342611, numbers[x]);
            break;
        }
    }
}

fn adds_upto(numbers: &Vec<i128>, target: i128, start: usize) -> (bool, usize) {
    let mut total = 0;
    for x in start..numbers.len() {
        total += numbers[x];
        if total == target {
            return (true, x);
        }
        if total > target {
            break;
        }
    }
    (false, 0)
}

#[test]
fn part_two() {
    let part_1_result = 257342611;
    let lines = utils::file_to_lines("input/09");
    let numbers: Vec<i128> = lines
        .into_iter()
        .map(|x| x.parse::<i128>().unwrap())
        .collect();

    for x in 0..numbers.len() {
        let (matches, end) = adds_upto(&numbers, part_1_result, x);
        if matches {
            let range = &numbers[x..end];
            let min = range.iter().min().unwrap();
            let max = range.iter().max().unwrap();
            let part_2_result = min + max;
            println!("DAY 09 : PART 2 : {}", part_2_result);
            assert_eq!(35602097, part_2_result);
            return;
        }
    }
}
