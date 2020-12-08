#[allow(unused_imports)]
use crate::utils;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

// Pt1: 151
// Pt2: 41559

#[test]
fn part_one() {
    let lines = utils::file_to_lines("input/07");
    let rules = load_rules(&lines);
    let all = get_all_containers(&"shiny gold".to_string(), &rules);
    // println!("{:#?}", all.len());
    assert_eq!(151, all.len());
    let count_within = count_all_bags_inside(&"shiny gold".to_string(), &rules);
    assert_eq!(41559, count_within);
}

fn count_all_bags_inside(color: &String, rules: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    let mut total = 0;
    if rules.contains_key(color) {
        for (qty, clr) in rules.get(color).unwrap() {
            total += qty * count_all_bags_inside(&clr, rules) + qty;
        }
    }
    total
}

// We start with rules for direct parent-child, after tha we go over each child and add that to parent
fn get_all_containers(color: &String, rules: &HashMap<String, Vec<(i32, String)>>) -> HashSet<String> {
    let mut containers = HashSet::new();
    let mut all_containers = HashSet::new();

    for (rcolor, contents) in rules {
        if contains(&color, &contents) {
            containers.insert(rcolor.clone());
            all_containers.insert(rcolor.clone());
        }
    }

    for item in containers {
        let sub_items = get_all_containers(&item, &rules);
        for new_item in sub_items {
            all_containers.insert(new_item);
        }
    }
    all_containers
}

fn contains(color: &String, contents: &Vec<(i32, String)>) -> bool {
    // let mut fount_match = false
    for (_, value) in contents {
        if value.eq(color) {
            return true;
        }
    }
    false
}

fn load_rules(lines: &Vec<String>) -> HashMap<String, Vec<(i32, String)>> {
    let mut bag_contents = HashMap::new();

    // bright purple bags contain 4 dark red bags, 5 clear white bags.
    let re = Regex::new(r"(\d \w+ \w+) ?bag").unwrap();
    let re_qty_color = Regex::new(r"(?P<qty>\d) (?P<color>\w+ \w+)").unwrap();

    for line in lines {
        if line.contains("contain no") {
            continue;
        }
        let parts: Vec<&str> = line.split("bags contain").collect();
        let mut contents = vec![];
        let result = re.find_iter(parts[1]);
        for r in result {
            let qty_color = re_qty_color.captures(r.as_str()).unwrap();
            let qty = qty_color["qty"].to_string().parse::<i32>().unwrap();
            let color = qty_color["color"].trim().to_string();
            contents.push((qty, color));
        }
        bag_contents.insert(parts[0].trim().to_string(), contents);
    }
    bag_contents
}

#[test]
fn part_two() {
    //Done in part 1
}
