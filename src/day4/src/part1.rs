use advent_of_code_2022::get_text_input;

struct SectionAssignment {
    start: u8,
    end: u8,
}

impl SectionAssignment {
    pub fn new(str: &str) -> Self {
        let mut split = str.split("-");
        let start = split.next().unwrap().parse().unwrap();
        let end = split.next().unwrap().parse().unwrap();
        return Self { start, end };
    }

    pub fn contains(&self, other: &Self) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }
}

fn main() {
    let input = get_text_input("4", "full");
    let pair_it = input.split("\n");
    let mut overlapped_assignments = 0;
    for pair in pair_it {
        let mut pair_split = pair.split(",");
        let first = SectionAssignment::new(pair_split.next().unwrap());
        let second = SectionAssignment::new(pair_split.next().unwrap());
        if first.contains(&second) || second.contains(&first) {
            overlapped_assignments += 1;
        }
    }
    println!("{overlapped_assignments}");
}
