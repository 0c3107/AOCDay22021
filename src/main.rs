/*
--- Day 2: Dive! ---

Now, you need to figure out how to pilot this thing.

It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:

    forward X increases the horizontal position by X units.
    down X increases the depth by X units.
    up X decreases the depth by X units.

Note that since you're on a submarine, down and up affect your depth, and so they have the opposite result of what you might expect.

The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:

forward 5
down 5
forward 8
up 3
down 8
forward 2

Your horizontal position and depth both start at 0. The steps above would then modify them as follows:

    forward 5 adds 5 to your horizontal position, a total of 5.
    down 5 adds 5 to your depth, resulting in a value of 5.
    forward 8 adds 8 to your horizontal position, a total of 13.
    up 3 decreases your depth by 3, resulting in a value of 2.
    down 8 adds 8 to your depth, resulting in a value of 10.
    forward 2 adds 2 to your horizontal position, a total of 15.

After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)

Calculate the horizontal position and depth you would have after following the planned course.
What do you get if you multiply your final horizontal position by your final depth?
*/
use std::fs;

#[derive(Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
pub struct Movement {
    direction: Direction,
    magnitude: usize,
}

pub struct FinalVector {
    distance: usize,
    depth: usize,
}

fn read_input_file() -> Vec<String> {
    let data = fs::read_to_string("resource/input").expect("Unable to read file");
    let split = data.split("\n");
    let mut data_vec: Vec<String> = Vec::new();
    for s in split {
        let s = s.trim();
        data_vec.push(s.to_string());
    }
    return data_vec;
}

fn split_string_by_whitespace(list: Vec<String>) -> Vec<String> {
    let mut split_list: Vec<String> = Vec::new();
    for s in list {
        let split = s.split(" ");
        for s in split {
            let s = s.trim();
            split_list.push(s.to_string());
        }
    }
    return split_list;
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
    let final_vector = FinalVector {
        distance,
        depth,
    };
    return final_vector;
}

fn main() {
    let split_list: Vec<String> = read_input_file();
    let split_list_split: Vec<String> = split_string_by_whitespace(split_list);
    let movement_vectors: Vec<Movement> = movement_vectors(split_list_split);
    let final_vector: FinalVector = final_vector_find(movement_vectors);
    let multiplied: usize = final_vector.depth * final_vector.distance;
    println!("Final vector:\nDistance: {}, Depth: {}.\nMultiplied they are: {}.", final_vector.distance, final_vector.depth, multiplied);
}
