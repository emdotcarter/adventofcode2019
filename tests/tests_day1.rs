macro_rules! fuel_counter_upper_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;

                let mut fuel_counter_upper = FuelCounterUpper::new();
                fuel_counter_upper.add_module(input);
                let actual = fuel_counter_upper.calculate_total_fuel_load();

                assert_eq!(expected, actual);
            }
        )*
    }
}

#[cfg(test)]
mod tests_day1 {
    use adventofcode2019::FuelCounterUpper;

    fuel_counter_upper_tests! {
        fcu_1: (14, 2),
        fcu_2: (1969, 966),
        fcu_3: (100756, 50346),
    }
}
