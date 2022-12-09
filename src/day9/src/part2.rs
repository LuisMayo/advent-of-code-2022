use std::{collections::HashSet, fmt::Write};

use advent_of_code_2022::get_text_input;
const KNOTS_NUMBER: usize = 10;

// x, y
#[derive(Clone, Copy)]
struct Coords {
    x: i16,
    y: i16,
}
#[derive(Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Coords {
    fn far_to(&self, other: &Self) -> bool {
        let x_difference = self.x.abs_diff(other.x);
        let y_difference = self.y.abs_diff(other.y);
        return x_difference > 1 || y_difference > 1;
    }

    fn move_instruction(&mut self, ins: &Instruction) {
        return self.move_direction(&ins.direction, ins.amount);
    }

    fn move_direction(&mut self, direction: &Direction, amount: i16) {
        match direction {
            Direction::UP => self.y += amount,
            Direction::DOWN => self.y -= amount,
            Direction::LEFT => self.x -= amount,
            Direction::RIGHT => self.x += amount,
        }
    }

    fn follow(&mut self, other: &Self) {
        let mut x_difference = other.x - self.x;
        if x_difference < -1 {
            x_difference = -1;
        }
        if x_difference > 1 {
            x_difference = 1;
        }
        let mut y_difference = other.y - self.y;
        if y_difference < -1 {
            y_difference = -1;
        }
        if y_difference > 1 {
            y_difference = 1;
        }
        self.x += x_difference;
        self.y += y_difference;
    }

    fn get_coords(&self) -> (i16, i16) {
        return (self.x, self.y);
    }
}

impl std::fmt::Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.x;
        let y = self.y;
        return write!(f, "{},{}", x, y);
    }
}
#[derive(Clone)]
struct Instruction {
    direction: Direction,
    amount: i16
}

impl Instruction {
    fn new(str: &str) -> Self {
        let mut split = str.split(" ");
        let temp_dir = split.next().unwrap();
        let temp_amount = split.next().unwrap();
        let full_dir;
        match temp_dir {
            "U" => full_dir = Direction::UP,
            "D" => full_dir = Direction::DOWN,
            "L" => full_dir = Direction::LEFT,
            "R" => full_dir = Direction::RIGHT,
            _ => panic!("Illegal direction in instruction")
        }
        let full_amount = temp_amount.parse().unwrap();
        return Self {
            direction: full_dir,
            amount: full_amount
        }
    }

    fn split_self(&self) -> Vec<Self> {
        let mut vec: Vec<Instruction> = Vec::new();
        let mut good = self.clone();
        good.amount = 1;
        for _ in 0..self.amount {
            vec.push(good.clone());
        }
        return vec;
    }
}

fn main() {
    let mut visited_locations = HashSet::new();
    let mut knots_arr = [Coords{x: 0, y: 0}; KNOTS_NUMBER];
    visited_locations.insert(knots_arr.last_mut().unwrap().get_coords());
    let input = get_text_input("9", "sample");
    let mut instructions_arr: Vec<Instruction> = Vec::new();
    for line in input.split("\n") {
        let instruction = Instruction::new(line);
        instructions_arr.append(&mut instruction.split_self());
    }
    for instruction in instructions_arr.iter() {
        let mut previous = None;
        for (idx, knot) in knots_arr.iter_mut().enumerate() {
            if idx == 0 {
                knot.move_instruction(instruction);
            } else if knot.far_to(previous.unwrap()){
                knot.follow(previous.unwrap());
            }
            previous = Some(knot);
        }
        visited_locations.insert(knots_arr.last_mut().unwrap().get_coords());
    }
    let length = visited_locations.len();
    println!("{length}");
}
