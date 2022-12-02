use advent_of_code_2022::get_text_input;
// use std::collections::HashMap;

const THEIR_ROCK: char = 'A';
const THEIR_PAPER: char = 'B';
const THEIR_SCISSORS: char = 'C';

const MY_ROCK: char = 'X';
const MY_PAPER: char = 'Y';
const MY_SCISSORS: char = 'Z';

enum MatchResult {
    LOSE = 0,
    DRAW = 3,
    WIN = 6
}

struct Match {
    my_choose: char,
    their_choose: char
}

impl Match {
    pub fn new (str: &str) -> Self {
        let mut char_iterator = str.chars();
        let their_choose = char_iterator.next().unwrap();
        let my_choose = char_iterator.skip(1).next().unwrap();
        return Self {my_choose, their_choose};
    }
    fn get_match_result(&self) -> MatchResult {
        let result: MatchResult;
        match self.my_choose {
            MY_ROCK => {
                match self.their_choose {
                    THEIR_ROCK => result = MatchResult::DRAW,
                    THEIR_PAPER => result = MatchResult::LOSE,
                    THEIR_SCISSORS => result = MatchResult::WIN,
                    _ => panic!("Ilegal move by them")
                }
            },
            MY_PAPER => {
                match self.their_choose {
                    THEIR_ROCK => result = MatchResult::WIN,
                    THEIR_PAPER => result = MatchResult::DRAW,
                    THEIR_SCISSORS => result = MatchResult::LOSE,
                    _ => panic!("Ilegal move by them")
                }
            },
            MY_SCISSORS => {
                match self.their_choose {
                    THEIR_ROCK => result = MatchResult::LOSE,
                    THEIR_PAPER => result = MatchResult::WIN,
                    THEIR_SCISSORS => result = MatchResult::DRAW,
                    _ => panic!("Ilegal move by them")
                }
            },
            _ => panic!("Ilegal move")
        }
        return result;
    }

    fn get_shape_score(&self) -> u32 {
        let rslt: u32;
        match self.my_choose {
            MY_ROCK => rslt = 1,
            MY_PAPER => rslt = 2,
            MY_SCISSORS => rslt = 3,
            _ => panic!("Ilegal move")
        }
        return rslt;
    }

    pub fn get_match_score(&self) -> u32 {
        return (self.get_match_result() as u32) + self.get_shape_score();
    }
}

fn main() {
    let input = get_text_input("2", "full");
    let matches_str = input.split("\n");
    let matches_arr = matches_str.map(|item| Match::new(item));
    let result = matches_arr.fold(0, |acc, matcch| matcch.get_match_score() + acc);
    println!("{result}");
}