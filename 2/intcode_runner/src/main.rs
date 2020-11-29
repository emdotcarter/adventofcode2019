#[derive(Debug)]
enum InputError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

fn reset_initial_intcode_state(noun: usize, verb: usize, intcode: &mut Vec<usize>) {
    intcode[1] = noun;
    intcode[2] = verb;
}

fn apply_command(command: usize, operands: &Vec<usize>) -> usize {
    match command {
        1 => return operands[0] + operands[1],
        2 => return operands[0] * operands[1],
        _ => panic!("Bad command code: {}", command),
    }
}

fn run_intcode(intcode: &mut Vec<usize>) {
    let mut command_index = 0;
    loop {
        let command = intcode[command_index];

        match command {
            99 => return,
            _ => {
                let operands = vec!(
                    intcode[intcode[command_index + 1]],
                    intcode[intcode[command_index + 2]],
                );

                let destination_index = intcode[command_index + 3];
                intcode[destination_index] = apply_command(command, &operands);

                command_index += 1 + operands.len() + 1;
            },
        }
    }
}

fn main() -> Result<(), InputError> {
    let args: Vec<String> = std::env::args().collect();
    let filepath = &args[1];

    let initial_intcode: Vec<usize> = std::fs::read_to_string(filepath).map_err(InputError::Io)?
        .split(",")
        .map(|value| value.trim_end())
        .map(|value| value.parse::<usize>())
        .map(|result| result.map_err(InputError::Parse).unwrap()) // TODO: Better way to handle this?
        .collect()
    ;

    let mut found = false;
    let mut found_noun = 0;
    let mut found_verb = 0;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode = initial_intcode.to_vec();
            reset_initial_intcode_state(noun, verb, &mut intcode);
            run_intcode(&mut intcode);

            if intcode[0] == 19690720 {
                found = true;
                found_noun = noun;
                found_verb = verb;

                break;
            }
        }

        if found {
            break;
        }
    }

    println!("100 * noun + verb: {}", 100 * found_noun + found_verb);

    Ok(())
}
