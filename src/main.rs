use std::fs;

pub enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Movement {
    direction: Direction,
    magnitude: usize,
}

pub struct FinalVector {
    distance: usize,
    depth: usize,
}

// Part 1 functions

fn read_input_file() -> Vec<String> {
    let data = fs::read_to_string("resource/input").expect("Unable to read file");
    let split = data.split('\n');
    let mut data_vec: Vec<String> = Vec::new();
    for s in split {
        let s = s.trim();
        data_vec.push(s.to_string());
    }
    data_vec
}

fn split_string_by_whitespace(list: Vec<String>) -> Vec<String> {
    let mut split_list: Vec<String> = Vec::new();
    for s in list {
        let split = s.split(' ');
        for s in split {
            let s = s.trim();
            split_list.push(s.to_string());
        }
    }
    split_list
}

fn movement_vectors(vec: Vec<String>) -> Vec<Movement> {
    let mut movement_vec: Vec<Movement> = Vec::new();
    for i in (1..=2000).step_by(2) {
        let direction_extract = vec[i - 1].clone();
        let direction: Direction = direction_detect(&direction_extract);
        let magnitude = vec[i].clone().parse::<usize>().unwrap();
        let next_input = Movement {
            direction,
            magnitude,
        };
        movement_vec.push(next_input);
    }
    movement_vec
}

pub fn direction_detect(str_input: &str) -> Direction {
    let direction_detected = match str_input {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
        _ => panic!("Invalid direction"),
    };
    direction_detected
}

fn final_vector_find(movements: Vec<Movement>) -> FinalVector {
    let mut distance: usize = 0;
    let mut depth: usize = 0;
    for i in 0..movements.len() {
        match movements[i].direction {
            Direction::Forward => distance += movements[i].magnitude,
            Direction::Down => depth += movements[i].magnitude,
            Direction::Up => depth -= movements[i].magnitude,
        }
    }
    FinalVector {
        distance,
        depth,
    }
}

// Part 2 code

fn final_vector_find_part_2(movements: Vec<Movement>) -> FinalVector {
    let mut aim: usize = 0;
    let mut depth: usize = 0;
    let mut distance: usize = 0;
    for i in 0..movements.len() {
        match movements[i].direction {
            Direction::Forward => {
                distance += movements[i].magnitude;
                depth += aim * movements[i].magnitude;
            },
            Direction::Down => aim += movements[i].magnitude,
            Direction::Up => aim -= movements[i].magnitude,
        }
    }
    FinalVector {
        distance,
        depth,
    }
}

fn main() {
    let split_list: Vec<String> = read_input_file();
    let split_list_split: Vec<String> = split_string_by_whitespace(split_list);
    let movement_vectors: Vec<Movement> = movement_vectors(split_list_split);
    //let final_vector_part_1: FinalVector = final_vector_find(movement_vectors);
    let final_vector: FinalVector = final_vector_find_part_2(movement_vectors);
    //let multiplied: usize = final_vector_part_1.depth * final_vector_part_1.distance;
    let multiplied2: usize = final_vector.depth * final_vector.distance;
    println!("Final vector:\nDistance: {}, Depth: {}.\nMultiplied they are: {}.", final_vector.distance, final_vector.depth, multiplied2);
}
