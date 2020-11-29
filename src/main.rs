use std::io::BufRead;

use adventofcode2019::InputError;
use adventofcode2019::FuelCounterUpper;

fn main() -> Result<(), InputError> {
    let args: Vec<String> = std::env::args().collect();
    let day = args[1].as_str();

    let result = match day {
        "day1" => run_day_1(&args[2..])?,
        "day2" => run_day_2(&args[2..])?,
        _ => panic!("Bad day argument: {}", day),
    };

    println!("Result: {}", result);

    Ok(())
}

fn run_day_1(args: &[String]) -> Result<i64, InputError> {
    println!("{}", args[0]);
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

    Ok(fuel_counter_upper.calculate_total_fuel_load())
}

fn run_day_2(args: &[String]) -> Result<i64, InputError> {
    Ok(0)
}
