use std::env;
use std::fs;

fn get_mutated_sequence(v: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut curr_op_code = v[0];
    let mut current_pos = 0;

    while curr_op_code != 99 {
        let l_value = v[v[current_pos + 1] as usize];
        let r_value = v[v[current_pos + 2] as usize];
        let output_pos = v[current_pos + 3] as usize;
        match curr_op_code {
            1 => v[output_pos] = l_value + r_value,
            2 => v[output_pos] = l_value * r_value,
            _ => println!("unknown op code {}", curr_op_code),
        }
        current_pos += 4;
        curr_op_code = v[current_pos];
    }

    v
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("No input file supplied!");
    }
    let filename = &args[1];
    println!("input file is: {}", filename);
    let file_content = fs::read_to_string(filename).expect("Could not open input file");

    let mut input_sequence: Vec<i32> = file_content
        .split(',')
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .collect();

    get_mutated_sequence(&mut input_sequence);

    println!("{:?}", input_sequence);
}

#[test]
fn test_input_sequence1() {
    let mut input = vec![1, 0, 0, 0, 99];
    println!("{:?}", get_mutated_sequence(&mut input));
    assert_eq!(input, [2, 0, 0, 0, 99]);
}

#[test]
fn test_input_sequence2() {
    let mut input = vec![2, 3, 0, 3, 99];
    println!("{:?}", get_mutated_sequence(&mut input));
    assert_eq!(input, [2, 3, 0, 6, 99]);
}

#[test]
fn test_input_sequence3() {
    let mut input = vec![2, 4, 4, 5, 99, 0];
    println!("{:?}", get_mutated_sequence(&mut input));
    assert_eq!(input, [2, 4, 4, 5, 99, 9801]);
}

#[test]
fn test_input_sequence4() {
    let mut input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    println!("{:?}", get_mutated_sequence(&mut input));
    assert_eq!(input, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
}
