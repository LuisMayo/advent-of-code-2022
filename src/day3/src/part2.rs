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

fn find_char_overlap(a: &str, b: &str, c: &str) -> char {
    let mut char_set = HashSet::new();
    a.chars().for_each(|char| {
        char_set.insert(char);
    });
    let found_duplicate = b.chars().filter(|char| char_set.get(char).is_some());
    let mut second_char_set = HashSet::new();
    found_duplicate.for_each(|char| {
        second_char_set.insert(char);
    });
    let badge_found = c.chars().find(|char| second_char_set.get(char).is_some());
    return badge_found.unwrap();
}

fn main() {
    let input = get_text_input("3", "full");
    let backpak_arr = input.split("\n").enumerate();
    let mut accumulated = 0;
    // let last_backpacks = Vec::new(3);
    let mut last_backpacks = [""; 3];
    for (i, backpack) in backpak_arr {
        let last_backpack_index = i % 3;
        last_backpacks[last_backpack_index] = backpack;
        if last_backpack_index == 2 {
            let char_overlap = find_char_overlap(last_backpacks[0], last_backpacks[1], last_backpacks[2]);
            accumulated += convert_char_to_priority(char_overlap);
        }
    }
    println!("{accumulated}");
}
