use std::{collections::HashSet, fmt::Write};

use advent_of_code_2022::get_text_input;

type VisitedPoint = bool;
// x, y
struct Coords {
    x: i8,
    y: i8,
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
        return x_difference > 1 || y_difference > 1 || (x_difference >= 1 && y_difference >= 1);
    }

    fn move_instruction(&mut self, ins: &Instruction) {
        return self.move_direction(&ins.direction, ins.amount);
    }

    fn move_direction(&mut self, direction: &Direction, amount: i8) {
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

    fn get_coords(&self) -> (i8, i8) {
        return (self.x, self.y)
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
    amount: i8
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
    let mut head = Coords{x: 0, y: 0};
    let mut tail = Coords{x: 0, y: 0};
    visited_locations.insert(tail.get_coords());
    let input = get_text_input("9", "sample");
    let mut instructions_arr: Vec<Instruction> = Vec::new();
    for line in input.split("\n") {
        let instruction = Instruction::new(line);
        instructions_arr.append(&mut instruction.split_self());
    }
    for instruction in instructions_arr.iter() {
        head.move_instruction(instruction);
        if tail.far_to(&head) {
            tail.follow(&head);
        }
        println!("{tail}");
        visited_locations.insert(tail.get_coords());
    }
    let length = visited_locations.len();
    println!("{length}");
}
