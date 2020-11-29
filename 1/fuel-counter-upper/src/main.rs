use std::io::BufRead;

#[derive(Debug)]
enum InputError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

fn calculate_needed_fuel(weight: i64) -> i64 {
    return weight / 3 - 2;
}

fn main() -> Result<(), InputError> {
    let module_weights_file = std::fs::File::open("./resources/module_weights.txt").map_err(InputError::Io)?;
    let module_weights_buf_reader = std::io::BufReader::new(module_weights_file);

    let mut total_fuel: i64 = 0;
    for weight in module_weights_buf_reader.lines() {
        let module_fuel = calculate_needed_fuel(
            weight
            .map_err(InputError::Io)?
            .parse()
            .map_err(InputError::Parse)?
        );
        total_fuel += module_fuel;

        let mut fuel_fuel: i64 = calculate_needed_fuel(module_fuel);
        while fuel_fuel > 0 {
            total_fuel += fuel_fuel;
            fuel_fuel = calculate_needed_fuel(fuel_fuel);
        }
    }

    println!("Total fuel needed: {}", total_fuel);

    Ok(())
}
