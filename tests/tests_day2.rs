macro_rules! intcode_computer_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;

                let mut intcode_computer = IntcodeComputer::new(input, &Vec::new());
                intcode_computer.run();
                let actual = intcode_computer.dump_program();

                assert_eq!(expected, actual);
            }
        )*
    }
}

#[cfg(test)]
mod tests_day2 {
    use adventofcode2019::IntcodeComputer;

    intcode_computer_tests! {
        icc_1: (&vec!(1, 0, 0, 0, 99), &vec!(2, 0, 0, 0, 99)),
        icc_2: (&vec!(2, 3, 0, 3, 99), &vec!(2, 3, 0, 6, 99)),
        icc_3: (&vec!(2, 4, 4, 5, 99, 0), &vec!(2, 4, 4, 5, 99, 9801)),
        icc_4: (&vec!(1, 1, 1, 4, 99, 5, 6, 0, 99), &vec!(30, 1, 1, 4, 2, 5, 6, 0, 99)),
    }
}
