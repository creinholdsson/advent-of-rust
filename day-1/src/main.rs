use std::env;
use std::fs;

fn get_fuel_amount(weight: i32) -> i32 {
    weight / 3 - 2
}

fn get_fuel_for_fuel(fuel: i32) -> i32 {
    let mut last_fuel_weight = fuel;
    let mut fuel_of_fuel_weights: Vec<i32> = vec![];
    while last_fuel_weight > 0 {
        last_fuel_weight = get_fuel_amount(last_fuel_weight);
        if last_fuel_weight > 0 {
            fuel_of_fuel_weights.push(last_fuel_weight);
        }
    }
    fuel_of_fuel_weights.iter().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("No input file supplied!");
    }
    let filename = &args[1];
    println!("input file is: {}", filename);

    let file_content = fs::read_to_string(filename).expect("Could not open input file");

    let mut fuel_amounts: Vec<i32> = vec![];

    for n in file_content.split('\n') {
        let weight = i32::from_str_radix(n, 10).unwrap();
        fuel_amounts.push(get_fuel_amount(weight));
        get_fuel_for_fuel(get_fuel_amount(weight));
    }

    let total_fuel: i32 = fuel_amounts.iter().sum();

    let total_fuel_for_fuel: i32 = fuel_amounts.iter().map(|&x| get_fuel_for_fuel(x)).sum();

    println!("Total fuel required: {}", total_fuel);
    println!("Total fuel for fuel: {}", total_fuel_for_fuel);
    println!("Gran total: {}", total_fuel + total_fuel_for_fuel);
}

#[test]
fn test_fuel_amount() {
    assert_eq!(get_fuel_amount(1969), 654);
    assert_eq!(get_fuel_amount(100756), 33583);
}

#[test]
fn test_fuel_of_fuel() {
    assert_eq!(get_fuel_for_fuel(654), 966 - 654);
}
