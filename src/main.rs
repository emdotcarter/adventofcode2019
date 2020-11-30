use std::io::BufRead;
use itertools::Itertools;

use adventofcode2019::InputError;
use adventofcode2019::FuelCounterUpper;
use adventofcode2019::IntcodeComputer;

fn main() -> Result<(), InputError> {
    let args: Vec<String> = std::env::args().collect();
    let day = args[1].as_str();

    let result = match day {
        "day1" => run_day_1(&args[2..])?,
        "day2" => run_day_2(&args[2..])?,
        _ => panic!("Bad day argument: {}", day),
    };

    print_result(result);

    Ok(())
}

fn print_result(result: std::collections::HashMap<&str, i64>) {
    println!("Results:");
    for (key, value) in result {
        println!("{}: {}", key, value);
    }
}

fn run_day_1(args: &[String]) -> Result<std::collections::HashMap<&str, i64>, InputError> {
    let module_weights_file = std::fs::File::open(&args[0]).map_err(InputError::Io)?;
    let module_weights_buf_reader = std::io::BufReader::new(module_weights_file);

    let mut fuel_counter_upper = FuelCounterUpper::new();
    for module_weight in module_weights_buf_reader.lines() {
        fuel_counter_upper.add_module(
            module_weight
            .map_err(InputError::Io)?
            .parse::<i64>().map_err(InputError::Parse)?
        );
    }

    Ok(
        [("fuel load", fuel_counter_upper.calculate_total_fuel_load())]
        .iter()
        .cloned()
        .collect()
    )
}

fn run_day_2(args: &[String]) -> Result<std::collections::HashMap<&str, i64>, InputError> {
    let intcode_filepath = &args[0];

    let initial_intcode: Vec<i64> = std::fs::read_to_string(intcode_filepath).map_err(InputError::Io)?
        .split(",")
        .map(|value| value.trim_end())
        .map(|value| value.parse::<i64>())
        .map(|result| result.map_err(InputError::Parse).unwrap()) // TODO: Better way to handle this?
        .collect()
    ;


    let mut computer = IntcodeComputer::new(&initial_intcode, &Vec::new());
    for override_vec in (0..2).map(|_| 0..100).multi_cartesian_product() {
        let overrides = vec!((1, override_vec[0]), (2, override_vec[1]));
        computer.reinitialize(&overrides);

        computer.run();

        if computer.output() == 19690720 {
            break;
        }
    }

    Ok(
        [("noun", computer.noun()), ("verb", computer.verb())]
        .iter()
        .cloned()
        .collect()
    )
}
