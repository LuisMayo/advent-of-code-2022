use std::collections::HashSet;

use advent_of_code_2022::get_text_input;

fn convert_char_to_priority(a: char) -> u32 {
    let ascii_number = a as u32;
    if ascii_number >= 97 && ascii_number <= 122 {
        // Undercase
        return ascii_number - 96;
    } else {
        // Uppercase (hopefully)
        return ascii_number - 38;
    }
}

fn find_char_overlap(a: &str, b: &str) -> char {
    let mut char_set = HashSet::new();
    a.chars().for_each(|char| {
        char_set.insert(char);
    });
    let found_duplicate = b.chars().find(|char| char_set.get(char).is_some());
    return found_duplicate.unwrap();
}

fn splice_backpack(backpack: &str) -> (&str, &str) {
    let middle = backpack.len() / 2;
    return (&backpack[..middle], &backpack[middle..]);
}

fn main() {
    let input = get_text_input("3", "full");
    let backpak_arr = input.split("\n");
    let mut accumulated = 0;
    for backpack in backpak_arr {
        let (part1, part2) = splice_backpack(backpack);
        let char_overlap = find_char_overlap(part1, part2);
        accumulated += convert_char_to_priority(char_overlap);
    }
    println!("{accumulated}");
}
