use std::collections::HashSet;

use advent_of_code_2022::get_text_input;

const SLIDING_WINDOW_SIZE: usize = 4;

fn check_buffer_is_different(buff: &[char]) -> bool {
    let mut set = HashSet::new();
    for char in buff {
        if set.contains(char) {
            return false;
        }
        set.insert(char);
    }
    return true;
}

fn main() {
    let mut sliding_window = [' '; SLIDING_WINDOW_SIZE];
    let input = get_text_input("6", "full");
    let char_it = input.chars().enumerate();
    let mut marker_index = 0;
    for (idx, char) in char_it {
        let sliding_window_pos = idx % SLIDING_WINDOW_SIZE;
        sliding_window[sliding_window_pos] = char;
        if idx >= 3 && check_buffer_is_different(&sliding_window) {
            marker_index = idx + 1;
            break;
        }
    }
    println!("{marker_index}");
}