use std::fs;

pub fn solution() {
    let input = fs::read_to_string("./input/day1.txt").unwrap();
    let parsed_lines = input.lines().map(|x| x.parse::<f64>().unwrap());
    let mut module_fuel_total = 0f64;
    let mut fuel_fuel_total = 0f64;
    for module_mass in parsed_lines {
        let fuel_required = calculate_mass_fuel_requirement(module_mass);
        module_fuel_total += fuel_required;
        fuel_fuel_total += calculate_fuel_requirements_for_fuel(fuel_required);
    }
    println!("Part One: {}", module_fuel_total);
    println!("Part Two: {}", module_fuel_total + fuel_fuel_total);
}


fn calculate_mass_fuel_requirement(module_mass: f64) -> f64 {
    ((module_mass / 3.0).floor() - 2.0) as f64
}

fn calculate_fuel_requirements_for_fuel(initial_fuel_mass: f64) -> f64 {
    let mut total_fuel_mass = 0f64;
    let mut prev_fuel_mass = initial_fuel_mass;
    loop {
        prev_fuel_mass = calculate_mass_fuel_requirement(prev_fuel_mass);
        if prev_fuel_mass <= 0f64 {
            return total_fuel_mass;
        }
        total_fuel_mass += prev_fuel_mass;
    }
}