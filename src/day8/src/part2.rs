use std::{slice::Iter, ops::Range, fmt::format};

use advent_of_code_2022::get_text_input;

type TreeHeight = u8;
// (row, column)

type TreeCoordinates = (usize, usize);
fn print_coords(coord: TreeCoordinates) -> String {
    let x = coord.0;
    let y= coord.1;
    return format!("({x},{y})");
}
type TreeCoordinatesSigned = (isize, isize);
enum Direction {
    UP, DOWN, LEFT, RIGHT
}

impl Direction {
    pub fn iterator() -> Iter<'static, Self> {
        static DIRECTIONS: [Direction; 4] = [Direction::UP, Direction::DOWN, Direction::LEFT, Direction::RIGHT];
        DIRECTIONS.iter()
    }
}

struct Forest {
    // Row, column
    trees: Vec<Vec<TreeHeight>>,
}

impl Forest {
    fn new(str: &str) -> Self {
        let mut forest = Vec::new();
        let rows = str.split("\n");
        for row in rows {
            let row_vec = row
                .chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect();
            forest.push(row_vec);
        }
        return Self { trees: forest };
    }

    fn are_coordinates_in_bounds(&self, coords: TreeCoordinatesSigned) -> bool {
        return coords.0 < self.trees.len().try_into().unwrap() && coords.0 >= 0 && coords.1 < self.trees[0].len().try_into().unwrap() && coords.1 >= 0;
    }

    fn are_coordinates_in_border(&self, coords: TreeCoordinates) -> bool {
        return coords.0 == self.trees.len() - 1 || coords.0 == 0 || coords.1 == self.trees[0].len() -1 || coords.1 == 0;
    }

    fn get_adjacent_tree_coordinates(&self, source: TreeCoordinates, direction: &Direction, depth: isize) -> TreeCoordinatesSigned {
        let source = (source.0.try_into().unwrap(), source.1.try_into().unwrap());
        match direction {
            Direction::UP => return (source.0 - depth, source.1),
            Direction::DOWN => return (source.0 + depth, source.1),
            Direction::LEFT => return (source.0, source.1 - depth),
            Direction::RIGHT => return (source.0, source.1 + depth)
        }
    }

    fn get_tree_height(&self, source: TreeCoordinates) -> u8 {
        return self.trees[source.0][source.1];
    }

    fn get_tree_scenic_view(&self, source: TreeCoordinates) -> u32 {
        if self.are_coordinates_in_border(source) {
            return 0;
        }
        let mut global_scenic_view = 1;
        for direction in Direction::iterator() {
            let mut current_dir_scenic_view = 0;
            let mut depth = 1;
            let mut next_tree_to_check = self.get_adjacent_tree_coordinates(source, direction, depth);
            while self.are_coordinates_in_bounds(next_tree_to_check) {
                current_dir_scenic_view += 1;
                let next_tree_usize = (next_tree_to_check.0.try_into().unwrap(), next_tree_to_check.1.try_into().unwrap());
                let source_dbg= print_coords(source);
                let next_dbg = print_coords(next_tree_usize);
                if self.get_tree_height(source) <= self.get_tree_height(next_tree_usize) {
                    break;
                }
                // println!("Tree {source_dbg} is NOT covered by {next_dbg}");
                depth+=1;
                next_tree_to_check = self.get_adjacent_tree_coordinates(source, direction, depth);
            }
            global_scenic_view *= current_dir_scenic_view;
        }
        return global_scenic_view;
    }

    fn get_all_candidate_trees(&self) -> (Range<usize>, Range<usize>) {
        return (1..self.trees.len() - 1, 1..self.trees[0].len() - 1);
    }
}

fn main() {
    let input = get_text_input("8", "full");
    let forest = Forest::new(&input);
    let candidates = forest.get_all_candidate_trees();
    let mut max_score = 0;
    for x in candidates.clone().0 {
        for y in candidates.clone().1 {
            println!("{x},{y}");
            let score = forest.get_tree_scenic_view((x, y));
            if score > max_score {
                max_score = score;
            }
        }
    }
    println!("{max_score}");
}
