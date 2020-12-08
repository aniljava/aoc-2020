#[allow(unused_imports)]
use crate::utils;

#[derive(Debug)]
struct Instruction {
    code: String,
    value: i32,
    execution_count: i32,
}

#[test]
fn part_one() {
    let mut instructions = load_instructions();
    let (accumulator, _) = run_program(&mut instructions);
    // println!("{}", accumulator);
    assert_eq!(1941, accumulator);
}

fn reset(instructions: &mut Vec<Instruction>) {
    for ins in instructions {
        ins.execution_count = 0;
    }
}

// accumulator, loops?
fn run_program(instructions: &mut Vec<Instruction>) -> (i32, bool) {
    reset(instructions);
    // println!("{:?}\n", instructions);
    let mut accumulator = 0;
    let mut position: i32 = 0;
    loop {
        if position >= instructions.len() as i32 {
            return (accumulator, false);
        }
        if position < 0 {
            return (accumulator, true);
        }

        let instruction = &mut instructions[position as usize];
        if instruction.execution_count > 0 {
            return (accumulator, true);
        }
        instruction.execution_count += 1;

        match instruction.code.as_str() {
            "jmp" => {
                position += instruction.value;
            }
            "acc" => {
                accumulator += instruction.value;
                position += 1;
            }
            "nop" => {
                position += 1;
            }
            _ => {}
        }
    }
}

fn load_instructions() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    for line in utils::file_to_lines("input/08") {
        let tokens: Vec<&str> = line.split(" ").collect();
        let code = tokens[0];
        let value = tokens[1].to_string().parse::<i32>().unwrap();
        let instruction = Instruction {
            code: code.to_string(),
            value: value,
            execution_count: 0,
        };
        instructions.push(instruction);
    }
    instructions
}

#[test]
fn part_two() {
    let mut instructions = load_instructions();
    let mut found = false;
    for i in 0..instructions.len() {
        let mut instruction = &mut instructions[i];
        match instruction.code.as_str() {
            "jmp" => {
                instruction.code = "nop".to_string();
            }
            "nop" => {
                instruction.code = "jmp".to_string();
            }
            "acc" => {}
            _ => {}
        }
        let (acc, loops) = run_program(&mut instructions);
        // println!("{}", acc);
        if !loops {
            found = true;
            assert_eq!(2096, acc);
            break;
        }

        let mut instruction = &mut instructions[i];
        match instruction.code.as_str() {
            "jmp" => {
                instruction.code = "nop".to_string();
            }
            "nop" => {
                instruction.code = "jmp".to_string();
            }
            "acc" => {}
            _ => {}
        }
    }
    assert!(found);
}
