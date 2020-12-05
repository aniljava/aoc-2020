use crate::utils;

#[test]
fn part_one() {
    let lines = utils::file_to_lines("input/01");
    let size = lines.len();

    let mut result = 0;
    for x in 0..size {
        for y in 0..size {
            if x == y {
                break;
            };

            let valx = lines[x].parse::<i32>().unwrap();
            let valy = lines[y].parse::<i32>().unwrap();
            let total = valx + valy;
            if total == 2020 {
                result = valx * valy;
                break;
            }
        }
    }
    assert_eq!(result, 365619);
    // Ok(())
}

#[test]
fn part_two() -> Result<(), Box<dyn std::error::Error>> {
    let lines = utils::file_to_lines("input/01");
    let size = lines.len();
    let mut result = 0;
    for x in 0..size {
        for y in 0..size {
            if x == y {
                break;
            };
            for z in 0..size {
                if x == y && x == z {
                    break;
                }
                let valx = lines[x].parse::<i32>().unwrap();
                let valy = lines[y].parse::<i32>().unwrap();
                let valz = lines[z].parse::<i32>().unwrap();
                let total = valx + valy + valz;
                if total == 2020 {
                    result = valx * valy * valz;
                    // dbg!(valx, valy, valz, valx * valy * valz);
                    break;
                }
            }
        }
    }
    assert_eq!(236873508, result);

    Ok(())
}