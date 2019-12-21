#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn draw_left(matrix: &mut Vec<Vec<char>>, x: usize, y: usize, steps: usize) {
    for pos in x - steps..x {
        matrix[y][pos] = match matrix[y][pos] {
            '.' => '-',
            '|' | '+' if pos == x - steps => '+',
            'o' => 'o',
            _ => 'X',
        };
    }
}

fn draw_right(matrix: &mut Vec<Vec<char>>, x: usize, y: usize, steps: usize) {
    for pos in x..x + steps {
        matrix[y][pos] = match matrix[y][pos] {
            '.' => '-',
            '|' | '+' if pos == x => '+',
            'o' => 'o',
            _ => 'X',
        };
    }
}

fn draw_up(matrix: &mut Vec<Vec<char>>, x: usize, y: usize, steps: usize) {
    for pos in y - steps..y {
        matrix[pos][x] = match matrix[pos][x] {
            '.' => '|',
            '-' | '+' if pos == y - steps => '+',
            'o' => 'o',
            _ => 'X',
        };
    }
}

fn draw_down(matrix: &mut Vec<Vec<char>>, x: usize, y: usize, steps: usize) {
    for pos in y..y + steps {
        matrix[pos][x] = match matrix[pos][x] {
            '.' => '|',
            '-' | '+' if pos == y => '+',
            'o' => 'o',
            _ => 'X',
        };
    }
}

fn initialize(matrix: &mut Vec<Vec<char>>) {
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            matrix[row][col] = '.';
        }
    }
}

fn find_crossings(matrix: &[Vec<char>]) -> Vec<Coordinate> {
    let mut result = vec![];

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            match matrix[y][x] {
                'x' | 'X' => result.push(Coordinate { x, y }),
                _ => {}
            }
        }
    }

    result
}

fn draw_lines(matrix: &mut Vec<Vec<char>>, lines: &[Vec<&str>], initial_pos: &Coordinate) {
    for line in lines.iter() {
        let mut x: usize = initial_pos.x;
        let mut y: usize = initial_pos.y;
        matrix[y][x] = 'o';
        for (line_counter, &step) in line.iter().enumerate() {
            if line_counter > 0 {
                matrix[y][x] = '+';
            }
            let steps: usize = step[1..].parse().unwrap();
            match &step.chars().nth(0) {
                Some('R') => {
                    draw_right(matrix, x, y, steps);
                    x += steps;
                }
                Some('L') => {
                    draw_left(matrix, x, y, steps);
                    x -= steps;
                }
                Some('U') => {
                    draw_up(matrix, x, y, steps);
                    y -= steps;
                }
                Some('D') => {
                    draw_down(matrix, x, y, steps);
                    y += steps;
                }
                Some(_) => {}
                None => {}
            }
        }
    }
}

fn draw_matrix(matrix: &[Vec<char>]) {
    for x in matrix.iter() {
        for y in x.iter() {
            print!("{}", y);
        }
        println!();
    }
}

fn get_best_score(crossings: &[Coordinate], central_point: &Coordinate) -> i32 {
    let mut best_score: i32 = std::i32::MAX;
    for n in crossings.iter() {
        let score = (n.x as i32 - central_point.x as i32).abs()
            + (n.y as i32 - central_point.y as i32).abs();
        if score < best_score {
            best_score = score;
        }
    }
    best_score
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("Expected file input");
    }

    let filename = &args[1];
    let filecontent = std::fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = filecontent.split('\n').collect();
    let mut line_vecs: Vec<Vec<&str>> = vec![];

    for line in lines {
        line_vecs.push(line.split(',').collect());
    }

    let mut state = vec![vec!['0'; 50000]; 50000];
    let central_point = Coordinate { x: 25000, y: 25000 };

    initialize(&mut state);
    draw_lines(&mut state, &line_vecs, &central_point);

    let crossings = find_crossings(&state);
    let best_score = get_best_score(&crossings, &central_point);

    println!("{}", best_score);
}

#[test]
fn test_simple1() {
    let mut state = vec![vec!['0'; 150]; 150];
    let steps = vec![vec!["R8", "U5", "L5", "D3"], vec!["U7", "R6", "D4", "L4"]];

    let central_point = Coordinate { x: 1, y: 8 };

    initialize(&mut state);
    draw_lines(&mut state, &steps, &central_point);
    let crossings = find_crossings(&state);
    let best_score = get_best_score(&crossings, &central_point);

    assert_eq!(6, best_score);
}

#[test]
fn test_simple2() {
    let mut state = vec![vec!['0'; 1000]; 1000];
    let steps = vec![
        vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
        vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
    ];

    let central_point = Coordinate { x: 450, y: 450 };

    initialize(&mut state);
    draw_lines(&mut state, &steps, &central_point);
    let crossings = find_crossings(&state);

    println!("{:?}", crossings);
    let best_score = get_best_score(&crossings, &central_point);

    assert_eq!(159, best_score);
}

#[test]
fn test_simple3() {
    let mut state = vec![vec!['0'; 10000]; 10000];
    let steps = vec![
        vec![
            "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
        ],
        vec![
            "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
        ],
    ];

    let central_point = Coordinate { x: 4500, y: 4500 };

    initialize(&mut state);
    draw_lines(&mut state, &steps, &central_point);
    let crossings = find_crossings(&state);

    println!("{:?}", crossings);
    let best_score = get_best_score(&crossings, &central_point);

    assert_eq!(135, best_score);
}
