#[allow(unused_imports)]
use crate::utils;
use regex::Regex;
use std::collections::HashMap;
use lazy_static::lazy_static;

#[test]
fn part_one() {
    let part_one = count_passport(false);
    assert_eq!(part_one, 235);
}

#[test]
fn part_two() {
    let part_two = count_passport(true);
    assert_eq!(part_two, 194);
}
fn count_passport(check_fields: bool) -> i32 {
    let mut total_passports = 0;
    let mut current_passport = HashMap::new();
    let lines = utils::file_to_lines("input/04");

    for line in lines {
        let line = line.trim();

        if line.is_empty() {
            if is_valid_passport(&current_passport, check_fields) {
                total_passports += 1;
            }
            current_passport.clear();
            continue;
        };

        let kvs: Vec<&str> = line.split(" ").collect();
        for pair in kvs {
            let name_value: Vec<&str> = pair.split(":").collect();
            let name = name_value[0].to_string();
            let value = name_value[1].to_string();
            current_passport.insert(name, value);
        }
    }
    //last line
    if is_valid_passport(&current_passport, check_fields) {
        total_passports += 1;
    }
    total_passports
}

fn is_valid_passport(map: &HashMap<String, String>, check_fields: bool) -> bool {
    let keys = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
    for key in keys.iter() {
        if map.contains_key(&key.to_string()) {
            if check_fields {
                if !validate_fields(map) {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
    true
}

fn validate_fields(map: &HashMap<String, String>) -> bool {
    lazy_static! {
        static ref HCL_REGEX: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        static ref ECL_REGEX: Regex =
            Regex::new(r"^amb$|^blu$|^brn$|^gry$|^grn$|^hzl$|^oth$").unwrap();
        static ref PID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }

    for key in map.keys() {
        let value = map.get(key).unwrap();
        match key.as_str() {
            "ecl" => {
                if !ECL_REGEX.is_match(value) {
                    return false;
                }
            }
            "pid" => {
                if !PID_REGEX.is_match(value) {
                    return false;
                }
            }
            "eyr" => {
                let i_value = value.parse::<i32>().unwrap();
                if i_value > 2030 || i_value < 2020 {
                    return false;
                }
            }
            "hcl" => {
                if !HCL_REGEX.is_match(value) {
                    return false;
                }
            }
            "byr" => {
                let i_value = value.parse::<i32>().unwrap();
                if i_value > 2002 || i_value < 1920 {
                    return false;
                }
            }
            "iyr" => {
                let i_value = value.parse::<i32>().unwrap();
                if i_value > 2020 || i_value < 2010 {
                    return false;
                }
            }
            "hgt" => {
                // hgt:61cm
                let unit = &value[value.len() - 2..];
                if !(value.contains("cm") || value.contains("in")) {
                    return false;
                }
                let height = value[0..value.len() - 2].parse::<i32>().unwrap();
                match unit {
                    "cm" => {
                        if height > 193 || height < 150 {
                            return false;
                        }
                    }
                    "in" => {
                        if height > 76 || height < 59 {
                            return false;
                        }
                    }
                    _ => {
                        // Error
                    }
                }
            }
            _ => {
                // unknown
            }
        }
    }
    true
}
