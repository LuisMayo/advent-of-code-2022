use std::collections::VecDeque;

use advent_of_code_2022::get_text_input;

type Stack = VecDeque<char>;

struct Instruction {
    pub repetations: u8,
    pub source: usize,
    pub dest: usize

}

impl Instruction {
    pub fn new (str: &str) -> Self{
        let mut char_it = str.split(" ").skip(1);
        let times: u8 = char_it.next().unwrap().parse().unwrap();
        let source: usize = char_it.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let dest: usize = char_it.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        return Self {
            repetations: times,
            source,
            dest
        }
    }

    fn move_crate(&self, stack_arr: &mut Vec<Stack>) {
        let mut temporal_stack = Vec::new();
        for _ in 0..self.repetations {
            let extracted = stack_arr.get_mut(self.source).unwrap().pop_back();
            match extracted {
                Some(val) => temporal_stack.push(val),
                None => break
            }
        }
        for temp_char in temporal_stack.iter().rev() {
            stack_arr.get_mut(self.dest).unwrap().push_back(*temp_char);
        }
    }
}

fn main() {
    let input = get_text_input("5", "full");
    let mut parts_it = input.split("\n\n");
    let initial_stack_info = parts_it.next().unwrap();
    let instructions_info = parts_it.next().unwrap();
    let initial_stack_vec = initial_stack_info.split("\n").collect::<Vec<&str>>();
    let initial_stack_it = initial_stack_vec.iter().rev().skip(1);
    let mut stacks_arr = Vec::new();
    for line in initial_stack_it {
        let char_it = line.chars().skip(1).step_by(4).enumerate();
        for (idx, char) in char_it {
            if char != ' ' {
                if stacks_arr.get(idx).is_none() {
                    stacks_arr.insert(idx, VecDeque::new());
                }
                stacks_arr.get_mut(idx).unwrap().push_back(char);
            }
        }
    }
    let instructions_it = instructions_info.split("\n").map(Instruction::new);
    for instruction in instructions_it {
        instruction.move_crate(&mut stacks_arr);
    }

    for stack in stacks_arr.iter_mut() {
        let char = stack.pop_back().unwrap_or(' ');
        print!("{char}");
    }
}