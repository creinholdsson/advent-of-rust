use std::env;
use std::fs;
use std::io;
use std::iter::FromIterator;

#[derive(PartialEq)]
#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate
}

#[derive(Debug)]
struct Operator {
    op_code: i32,
    param1: ParameterMode,
    param2: ParameterMode,
    param3: ParameterMode
}

impl ParameterMode {
    fn from_char(x: char) -> ParameterMode {
        println!("evaluating from char");
        match x {
            '1' => ParameterMode::Immediate,
            _ => ParameterMode::Position
        }
    }
}

fn get_operator(op_codes: i32) -> Operator {
    let chars: Vec<char> = op_codes.to_string().chars().collect();

    let op_code = match chars.len() {
        
        2..=1000 => i32::from_str_radix(&String::from_iter(chars[chars.len()-2 as usize..].iter()), 10).unwrap(),
        _ => chars[chars.len() - 1 as usize].to_digit(10).unwrap() as i32
    };

    let param1_mode = match chars.len() {
        3..=1000 => ParameterMode::from_char(chars[chars.len() - 3 as usize]),
        _ => ParameterMode::Position
    };

    let param2_mode = match chars.len() {
        4..=1000 => ParameterMode::from_char(chars[chars.len() - 4 as usize]),
        _ => ParameterMode::Position
    };

    let param3_mode = match chars.len() {
        5..=1000 => ParameterMode::from_char(chars[chars.len() - 5 as usize]),
        _ => ParameterMode::Position
    };

    Operator { op_code, param1: param1_mode, param2: param2_mode, param3: param3_mode}
}

fn get_mutated_sequence(v: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut curr_op_code = get_operator(v[0]);
    println!("{:?}", curr_op_code);
    let mut current_pos = 0;

    while curr_op_code.op_code != 99 {
        let mut next_pos: usize = current_pos;
        match curr_op_code.op_code {
            1 => next_pos = add_instruction(v, current_pos, &curr_op_code),
            2 => next_pos = mult_instruction(v, current_pos, &curr_op_code),
            3 => next_pos = input_instruction(v, current_pos),
            4 => next_pos = output_instruction(v, current_pos),
            _ => println!("unknown op code {}", curr_op_code.op_code),
        }
        current_pos = next_pos;
        curr_op_code = get_operator(v[current_pos]);
        println!("{:?}", curr_op_code);
    }

    v
}

fn add_instruction(v: &mut Vec<i32>, current_pos: usize, operator: &Operator) -> usize {
    let l_value = match operator.param1 {
        ParameterMode::Position => v[v[current_pos + 1] as usize],
        ParameterMode::Immediate => v[current_pos + 1]
    };
     
    let r_value = match operator.param2 {
        ParameterMode::Position => v[v[current_pos + 2] as usize],
        ParameterMode::Immediate => v[current_pos + 2]
    };
    let output_pos = match operator.param3 {
        ParameterMode::Position => v[current_pos + 3] as usize,
        ParameterMode::Immediate => v[current_pos + 3] as usize
    };
    v[output_pos] = l_value + r_value;
    current_pos + 4
}

fn mult_instruction(v: &mut Vec<i32>, current_pos: usize, operator: &Operator) -> usize {
    let l_value = match operator.param1 {
        ParameterMode::Position => v[v[current_pos + 1] as usize],
        ParameterMode::Immediate => v[current_pos + 1]
    };
     
    let r_value = match operator.param2 {
        ParameterMode::Position => v[v[current_pos + 2] as usize],
        ParameterMode::Immediate => v[current_pos + 2]
    };
    let output_pos = match operator.param3 {
        ParameterMode::Position => v[current_pos + 3] as usize,
        ParameterMode::Immediate => v[current_pos + 3] as usize
    };
    v[output_pos] = l_value * r_value;
    current_pos + 4
}

fn input_instruction(v: &mut Vec<i32>, current_pos: usize) -> usize {
    println!("Input integer: ");
    let mut input = String::new();
    let output_pos = v[current_pos + 1] as usize;
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input  = i32::from_str_radix(&input.trim(), 10).unwrap();
            v[output_pos] = input;
        },
        Err(error) => panic!(error)
    }
    current_pos + 1
}

fn output_instruction(v: &Vec<i32>, current_pos: usize) -> usize {
    println!("Program output: {}", v[current_pos + 1]);
    current_pos + 1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("No input file supplied!");
    }
    let filename = &args[1];
    println!("input file is: {}", filename);
    let file_content = fs::read_to_string(filename).expect("Could not open input file");

    let input_sequence: Vec<i32> = file_content
        .split(',')
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .collect();

    println!("{:?}", input_sequence);

    get_mutated_sequence(&mut input_sequence.clone());
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

#[test]
fn test_op_code_parser() {
    let op = get_operator(1002);

    assert_eq!(2, op.op_code);
    assert_eq!(ParameterMode::Position, op.param1);
    assert_eq!(ParameterMode::Immediate, op.param2);
    assert_eq!(ParameterMode::Position, op.param3);
}