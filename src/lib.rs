#[derive(Debug)]
pub enum InputError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

#[derive(PartialEq, Eq, Hash)]
pub struct Module {
    id: String,
    weight: i64,
}

pub struct FuelCounterUpper {
    module_fuel_loads: std::collections::HashMap<Module, i64>,
}

struct IntcodeInstruction  {
    opcode: i64,
    operand_count: i64,
    has_output_address: bool,
    calculation: std::boxed::Box<dyn Fn(&Vec<i64>) -> i64>,
    is_terminator: bool,
}

pub struct IntcodeComputer {
    instructions: std::collections::HashMap<i64, IntcodeInstruction>,
    initial_program: Vec<i64>,
    program: Vec<i64>,
    overrides: Vec<(i64, i64)>,
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

    pub fn calculate_total_fuel_load(&self) -> i64 {
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

impl IntcodeComputer {
    fn generate_instructions() -> std::collections::HashMap<i64, IntcodeInstruction> {
        let instructions = vec!(
            IntcodeInstruction {
                opcode: 1,
                operand_count: 2,
                has_output_address: true,
                calculation: std::boxed::Box::new(|v: &Vec<i64>| -> i64 { v[0] + v[1] }),
                is_terminator: false,
            },
            IntcodeInstruction {
                opcode: 2,
                operand_count: 2,
                has_output_address: true,
                calculation: std::boxed::Box::new(|v: &Vec<i64>| -> i64 { v[0] * v[1] }),
                is_terminator: false,
            },
            IntcodeInstruction {
                opcode: 99,
                operand_count: 0,
                has_output_address: false,
                calculation: std::boxed::Box::new(|_v: &Vec<i64>| -> i64 { return 0 }),
                is_terminator: true,
            },
        );

        let mut hash_map = std::collections::HashMap::new();
        for instruction in instructions {
            hash_map.insert(instruction.opcode, instruction);
        }

        return hash_map;
    }

    pub fn new(program: &Vec<i64>, overrides: &Vec<(i64, i64)>) -> IntcodeComputer {
        let initialized_program = program.to_vec();
        let mut computer = IntcodeComputer {
            instructions: IntcodeComputer::generate_instructions(),
            initial_program: program.to_vec(),
            program: initialized_program,
            overrides: overrides.to_vec(),
        };
        computer.apply_overrides();

        return computer;
    }

    pub fn reinitialize(&mut self, overrides: &Vec<(i64, i64)>) {
        self.program = self.initial_program.to_vec();
        self.overrides= overrides.to_vec();

        self.apply_overrides();
    }

    fn apply_overrides(&mut self) {
        for (location, value) in &self.overrides {
            self.program[*location as usize] = *value;
        }
    }

    pub fn run(&mut self) {
        let mut execution_index = 0;
        loop {
            let instruction: &IntcodeInstruction = &self.instructions[&self.program[execution_index]];

            if instruction.is_terminator {
                break;
            }

            let mut operands = Vec::<i64>::new();
            for i in 1..instruction.operand_count + 1 {
                let operand_index = self.program[execution_index + i as usize];
                operands.push(self.program[operand_index as usize]);
            }
            let result = (instruction.calculation)(&operands);

            let mut new_execution_index = execution_index + 1 + instruction.operand_count as usize;
            if instruction.has_output_address {
                let output_address_index = self.program[execution_index + instruction.operand_count as usize + 1];
                self.program[output_address_index as usize] = result;
                new_execution_index += 1;
            }

            execution_index = new_execution_index;
        }
    }

    pub fn output(&self) -> i64 {
        return self.program[0];
    }

    pub fn noun(&self) -> i64 {
        return self.program[1];
    }

    pub fn verb(&self) -> i64 {
        return self.program[2];
    }

    pub fn dump_program(&self) -> &Vec<i64> {
        return &self.program;
    }

}
