use std::{fs, env};

pub fn get_text_input(day: &str, part: &str) -> String {
    let exec_path = env::current_exe().unwrap();
    // let exec_name = exec_path.file_name().unwrap().to_str().unwrap();
    // let part_index = exec_name.chars().position(|c| c == 'p').unwrap();
    // let day = exec_name.substring(1, part_index);
    // let input_path = d
    let computed_part;
    match env::args_os().nth(1) {
        Some(val) => match val.to_str() {
            Some(val) => computed_part = String::from(val),
            None => computed_part = String::from(part)
        },
        None => computed_part = String::from(part)
    }
    return fs::read_to_string(format!("./src/day{day}/input/{computed_part}.txt")).unwrap().replace("\r", "");
}
