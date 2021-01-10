use std::fs;

pub fn solution() {
    let input_str = fs::read_to_string("./input/day2.txt").unwrap();
    let mut input: Vec<u32> = Vec::new();
    for num_str in input_str.split(',') {
        input.push(num_str.parse().unwrap());
    }
    match get_noun_and_verb(&input, 19690720) {
        Some((noun, verb)) => println!("The number is {}", noun * 100 + verb),
        None => println!("Did not get any value"),
    }
}

fn get_noun_and_verb(intcode: &Vec<u32>, expected: u32) -> Option<(u32, u32)> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode_clone = intcode.clone();
            if let Ok(res) = get_output(&mut intcode_clone, noun, verb) {
                if expected == res {
                    return Some((noun, verb));
                }
            }
        }
    }
    return None;
}

fn get_output(intcode: &mut Vec<u32>, noun: u32, verb: u32) -> Result<u32, IntcodeError> {
    intcode[1] = noun;
    intcode[2] = verb;
    let mut instruction_pointer = 0usize;
    loop {
        let op_code = intcode
            .get(instruction_pointer)
            .ok_or(IntcodeError::AccessOutOfRange)?;
        match op_code {
            1 => {
                let param_one = *intcode
                    .get(instruction_pointer + 1)
                    .ok_or(IntcodeError::AccessOutOfRange)?
                    as usize;
                let param_two = *intcode
                    .get(instruction_pointer + 2)
                    .ok_or(IntcodeError::AccessOutOfRange)?
                    as usize;
                let param_three = *intcode
                    .get(instruction_pointer + 3)
                    .ok_or(IntcodeError::AccessOutOfRange)?
                    as usize;
                let value_one = *intcode
                    .get(param_one)
                    .ok_or(IntcodeError::AccessOutOfRange)?;
                let value_two = *intcode
                    .get(param_two)
                    .ok_or(IntcodeError::AccessOutOfRange)?;
                let value_three = intcode
                    .get_mut(param_three)
                    .ok_or(IntcodeError::AccessOutOfRange)?;
                *value_three = value_one + value_two;
                instruction_pointer += 4;
            }
            2 => {
                let param_one = *intcode
                    .get(instruction_pointer + 1)
                    .ok_or(IntcodeError::AccessOutOfRange)?
                    as usize;
                let param_two = *intcode
                    .get(instruction_pointer + 2)
                    .ok_or(IntcodeError::AccessOutOfRange)?
                    as usize;
                let param_three = *intcode
                    .get(instruction_pointer + 3)
                    .ok_or(IntcodeError::AccessOutOfRange)?
                    as usize;
                let value_one = *intcode
                    .get(param_one)
                    .ok_or(IntcodeError::AccessOutOfRange)?;
                let value_two = *intcode
                    .get(param_two)
                    .ok_or(IntcodeError::AccessOutOfRange)?;
                let value_three = intcode
                    .get_mut(param_three)
                    .ok_or(IntcodeError::AccessOutOfRange)?;
                *value_three = value_one * value_two;
                instruction_pointer += 4;
            }
            99 => {
                return Ok(intcode[0]);
            }
            _ => return Err(IntcodeError::InvalidOpCode),
        }
    }
}

#[derive(Debug)]
enum IntcodeError {
    AccessOutOfRange,
    InvalidOpCode,
}
