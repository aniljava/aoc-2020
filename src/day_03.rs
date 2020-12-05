use crate::utils;

#[test]
fn part_one() {
    let lines = utils::file_to_lines("input/03");
    let total = count(&lines, 3, 1);
    assert_eq!(187, total);
}
#[test]
fn part_two() {
    let lines = utils::file_to_lines("input/03");
    let pairs = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let counts: Vec<usize> = pairs.iter().map(|&p| count(&lines, p.0, p.1)).collect();
    let product: usize = counts.iter().product();
    assert_eq!(4723283400, product);
}

fn count(lines: &Vec<String>, right: usize, down: usize) -> usize {
    let line_count = lines.len();
    let line_length = lines[0].len();
    let mut total_trees = 0;

    let mut current_line = 0;
    let mut rpos = 0;
    while current_line < line_count - down {
        current_line += down;
        rpos = (rpos + right) % line_length;
        let item = lines[current_line].chars().nth(rpos).unwrap();
        if item == '#' {
            total_trees += 1;
        }
    }
    total_trees
}
