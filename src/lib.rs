#[derive(Debug)]
pub enum InputError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Input(String),
}

#[derive(PartialEq, Eq, Hash)]
pub struct Module {
    id: String,
    weight: i64,
}

pub struct FuelCounterUpper {
    module_fuel_loads: std::collections::HashMap<Module, i64>,
}

impl Module {
    pub fn new(weight: i64) -> Module {
        return Module {
            id: uuid::Uuid::new_v4().to_string(),
            weight: weight,
        };
    }
}

impl FuelCounterUpper {
    pub fn new() -> FuelCounterUpper {
        return FuelCounterUpper {
            module_fuel_loads: std::collections::HashMap::new(),
        };
    }

    pub fn add_module(&mut self, module_weight: i64) {
        self.module_fuel_loads.insert(
            Module::new(module_weight),
            self.calculate_module_fuel_load(module_weight),
        );
    }

    pub fn calculate_total_fuel_load(self) -> i64 {
        return self.module_fuel_loads.values().fold(0, |accumulator, fuel_load| accumulator + fuel_load);
    }

    fn calculate_module_fuel_load(&self, module_weight: i64) -> i64 {
        let fuel_for_weight = |weight: i64| -> i64 { weight / 3 - 2 };

        let mut total_fuel = fuel_for_weight(module_weight);

        let mut fuel_for_fuel: i64 = fuel_for_weight(total_fuel);
        while fuel_for_fuel > 0 {
            total_fuel += fuel_for_fuel;
            fuel_for_fuel = fuel_for_weight(fuel_for_fuel);
        }

        return total_fuel;
    }
}
