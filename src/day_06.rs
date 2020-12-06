#[allow(unused_imports)]
use crate::utils;
use std::collections::HashMap;

#[test]
fn part_one() {
    let content = utils::file_to_string("input/06");
    let chunks: Vec<&str> = content.split("\n\n").collect();

    let mut total = 0;
    let mut total_all = 0;
    for chunk in chunks {
        let flat = chunk.replace("\n", "");
        let mut uniques = HashMap::new();
        for c in flat.chars() {
            uniques.insert(c, 1);
        }
        total += uniques.keys().len();

        let lines: Vec<&str> = chunk.split("\n").collect();
        for k in uniques.keys() {
            let mut exists = true;
            for line in &lines {
                if !line.contains(&k.to_string()) {
                    exists = false;
                    break;
                }
            }
            if exists {
                total_all += 1;
            }
        }
    }
    // println!("{}", total_all);
    // println!("{}", total);
    assert_eq!(total, 6382);
    assert_eq!(total_all, 3197);
}

#[test]
fn part_two() {
    //implemented in part 1
}
