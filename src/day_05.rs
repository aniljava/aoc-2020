#[allow(unused_imports)]
use crate::utils;
use std::collections::HashMap;

#[test]

fn part_one() {
    let lines = utils::file_to_lines("input/05");
    let mut map = HashMap::new();

    for line in lines {
        let num = bsp_to_num(&line);
        map.insert(num, "occupied");
    }
    let max = map.keys().max().unwrap();
    assert_eq!(max, &998);

    // 998 is max and input is 960 total entries, so starts at 38
    for x in 38..998 {
        if !map.contains_key(&x) {
            assert_eq!(x, 676);
        }
    }
}

fn bsp_to_num(bsp: &String) -> i32 {
    let chars: Vec<char> = bsp.chars().collect();
    let rows = &chars[0..7];
    let cols = &chars[7..];

    let mut left = 0;
    let mut right = 128;
    let mut middle = (right - left) / 2;

    for c in rows {
        match c {
            'F' => {
                // Lower
                right = middle;
                middle = (((right - left) as f32 / 2.0).floor()) as i32 + left;
            }
            'B' => {
                // Upper
                left = middle;
                middle = (((right - left) as f32 / 2.0).floor()) as i32 + left;
            }
            _ => {}
        }
    }
    let row = middle;

    let mut left = 0;
    let mut right = 8;
    let mut middle = (right - left) / 2;

    for c in cols {
        match c {
            'L' => {
                // Lower
                right = middle;
                middle = (((right - left) as f32 / 2.0).floor()) as i32 + left;
            }
            'R' => {
                // Upper
                left = middle;
                middle = (((right - left) as f32 / 2.0).floor()) as i32 + left;
            }
            _ => {}
        }
    }
    let col = middle;
    row * 8 + col
}

#[test]
fn part_two() {
    // Implemented in part 1
}
