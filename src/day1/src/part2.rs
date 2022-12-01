use advent_of_code_2022::get_text_input;
fn main() {
    let input = get_text_input("1", "full");
    let arr_of_elves = input.split("\n\n");
    let mut max_map: Vec<u32> = arr_of_elves.map(|elve| {
        let snacks_arr = elve.split("\n");
        let calories_count = snacks_arr.fold(0, |calories, snack| {
            let current_calories = snack.parse::<u32>().unwrap_or(0);
            return current_calories + calories;
        });
        return calories_count;
    }).collect();
    //
    max_map.sort_unstable();
    let len = max_map.len();
    let top_three_sum: u32 = max_map[len-3..].iter().sum();
    println!("{top_three_sum}");
}
