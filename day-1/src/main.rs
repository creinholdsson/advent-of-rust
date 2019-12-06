use std::env;
use std::fs;

fn get_fuel_amount(weight: i32) -> i32 {
    return weight / 3 -2;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("No input file supplied!");
    }
    let filename = &args[1];
    println!("input file is: {}", filename);

    let file_content = fs::read_to_string(filename).expect("Could not open input file");

    let mut fuel_amounts: Vec<i32> = vec!();

    for n in file_content.split("\n") {
        let weight = i32::from_str_radix(n, 10).unwrap();
        fuel_amounts.push(get_fuel_amount(weight));
    }

    let total_fuel: i32 = fuel_amounts.iter().sum();

    println!("Total fuel required: {}", total_fuel);
}

#[test]
fn test_fuel_amount() {
    assert_eq!(get_fuel_amount(1969), 654);
    assert_eq!(get_fuel_amount(100756), 33583);
}
