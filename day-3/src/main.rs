#[derive(Debug, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y && self.x == other.x
    }
}

fn find_crossings_from_coordinates(line1: &[Coordinate], line2: &[Coordinate]) -> Vec<Coordinate> {
    let mut result: Vec<Coordinate> = vec![];
    for coord in line1.iter() {
        if line2.iter().filter(|&x| *x == *coord).count() > 0 {
            result.push(coord.clone());
        }
    }
    result
}

fn get_best_score(crossings: &[Coordinate], central_point: &Coordinate) -> i32 {
    let mut best_score: i32 = std::i32::MAX;
    for n in crossings.iter() {
        if *n == *central_point {
            continue;
        }
        let score = (n.x as i32 - central_point.x as i32).abs()
            + (n.y as i32 - central_point.y as i32).abs();
        if score < best_score {
            best_score = score;
        }
    }
    best_score
}

fn get_coordinates_from_sequence(line: &[&str], central_point: &Coordinate) -> Vec<Coordinate> {
    let mut coordinates: Vec<Coordinate> = vec![];
    let mut current_pos = central_point.clone();
    coordinates.push(current_pos.clone());
    for &step in line.iter() {
        let steps: usize = step[1..].parse().unwrap();
        match &step.chars().nth(0) {
            Some('R') => {
                for _ in 0..steps {
                    current_pos.x += 1;
                    coordinates.push(current_pos.clone());
                }
            }
            Some('L') => {
                for _ in 0..steps {
                    current_pos.x -= 1;
                    coordinates.push(current_pos.clone());
                }
            }
            Some('U') => {
                for _ in 0..steps {
                    current_pos.y -= 1;
                    coordinates.push(current_pos.clone());
                }
            }
            Some('D') => {
                for _ in 0..steps {
                    current_pos.y += 1;
                    coordinates.push(current_pos.clone());
                }
            }
            Some(_) | None => panic!("Failed to parse move!"),
        }
    }
    coordinates
}

fn get_steps_to_coordinate(
    line: &[Coordinate],
    coordinate: &Coordinate,
) -> Result<i32, &'static str> {
    match line
        .iter()
        .position(|x| x.x == coordinate.x && x.y == coordinate.y)
    {
        Some(x) => Ok(x as i32),
        None => Err("Not found on line"),
    }
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

    let central_point = Coordinate { x: 25000, y: 25000 };

    let line1 = get_coordinates_from_sequence(&line_vecs[0], &central_point);
    let line2 = get_coordinates_from_sequence(&line_vecs[1], &central_point);
    let crossings = find_crossings_from_coordinates(&line1, &line2);
    let best_score = get_best_score(&crossings, &central_point);
    let mut steps_counts: Vec<i32> = vec![];

    for crossing in crossings.iter() {
        if *crossing == central_point {
            continue;
        }
        let steps_line_1 = get_steps_to_coordinate(&line1, crossing).unwrap();
        let steps_line_2 = get_steps_to_coordinate(&line2, crossing).unwrap();
        steps_counts.push(steps_line_1 + steps_line_2);
    }

    let min = *steps_counts.iter().min().unwrap();

    println!("Best manhattan distance: {}", best_score);
    println!("Best step count: {}", min);
}

#[test]
fn test_get_all_coordinates() {
    let steps = vec![
        vec![
            "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
        ],
        vec![
            "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
        ],
    ];

    let central_point = Coordinate { x: 3000, y: 3000 };

    let line1 = get_coordinates_from_sequence(&steps[0], &central_point);
    let line2 = get_coordinates_from_sequence(&steps[1], &central_point);
    let crossings = find_crossings_from_coordinates(&line1, &line2);

    let best_score = get_best_score(&crossings, &central_point);

    let mut steps_counts: Vec<i32> = vec![];

    for crossing in crossings.iter() {
        if *crossing == central_point {
            continue;
        }
        let steps_line_1 = get_steps_to_coordinate(&line1, crossing).unwrap();
        let steps_line_2 = get_steps_to_coordinate(&line2, crossing).unwrap();
        steps_counts.push(steps_line_1 + steps_line_2);
        println!("Steps: {}", steps_line_1 + steps_line_2);
    }
    assert_eq!(410, *steps_counts.iter().min().unwrap());
    assert_eq!(135, best_score);
}

#[test]
fn test_get_all_coordinates2() {
    let steps = vec![
        vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
        vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
    ];

    let central_point = Coordinate { x: 3000, y: 3000 };

    let line1 = get_coordinates_from_sequence(&steps[0], &central_point);
    let line2 = get_coordinates_from_sequence(&steps[1], &central_point);
    let crossings = find_crossings_from_coordinates(&line1, &line2);
    let best_score = get_best_score(&crossings, &central_point);

    let mut steps_counts: Vec<i32> = vec![];

    for crossing in crossings.iter() {
        if *crossing == central_point {
            continue;
        }
        let steps_line_1 = get_steps_to_coordinate(&line1, crossing).unwrap();
        let steps_line_2 = get_steps_to_coordinate(&line2, crossing).unwrap();
        steps_counts.push(steps_line_1 + steps_line_2);
    }
    assert_eq!(610, *steps_counts.iter().min().unwrap());
    assert_eq!(159, best_score);
}
