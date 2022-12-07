use advent_of_code_2022::get_text_input;

const TOTAL_SPACE:u32 = 70000000;
const NEEDED_SPACE:u32 = 30000000;

#[derive(PartialEq)]
enum FileType {
    DIR,
    FILE,
}

struct FileNode {
    file_type: FileType,
    name: String,
    size: Option<u32>,
    children: Vec<Self>,
}

impl FileNode {
    fn get_total_size(&self) -> u32 {
        let mut accum = self.size.unwrap_or(0);
        for child in self.children.iter() {
            accum += child.get_total_size();
        }
        return accum;
    }

    fn get_child_by_name(&mut self, name: &str) -> Option<&mut Self> {
        return self.children.iter_mut().find(|item| item.name == name);
    }

    fn get_all_directories(&self) -> Vec<&FileNode> {
        let mut full_dir_arr = self.children.iter().filter(|child| child.file_type == FileType::DIR).map(|children| children).collect::<Vec<&FileNode>>();
        // full_dir_arr.clone_from_slice(&self.children);
        for child in self.children.iter() {
            let mut child_dirs = child.get_all_directories();
            full_dir_arr.append(&mut child_dirs);
        }
        return full_dir_arr;
    }
}

fn get_node_by_history<'a>(root_node: &'a mut FileNode, history: &Vec<String>) -> &'a mut FileNode {
    let mut node = root_node;
    for entry in history {
        node = node.get_child_by_name(entry.as_str()).unwrap();
    }
    return node;
}

// struct ShellState<'a> {
//     current_node: &'a FileNode
// }

fn main() {
    let input = get_text_input("7", "full");
    let instructions_it = input.split("$");
    let mut root_node = FileNode {
        name: String::new(),
        file_type: FileType::DIR,
        size: None,
        children: Vec::new(),
    };
    let mut current_node = &mut root_node;
    let mut history = Vec::new();
    for instruction in instructions_it.map(|instruction| instruction.trim()).filter(|instruction| instruction.len() != 0) {
        if instruction.starts_with("cd") {
            let mut args_split = instruction.split(" ");
            let child_path = args_split.nth(1).unwrap();
            if child_path == "/" {
                current_node = &mut root_node;
                history.clear();
            } else if child_path == ".." {
                history.pop();
                current_node = get_node_by_history(&mut root_node, &history);
            } else {
                current_node = current_node.get_child_by_name(child_path).unwrap();
                history.push(String::from(child_path));
            }
        } else {
            // ls
            let rows = instruction.split("\n").skip(1);
            for row in rows {
                let mut dir_info = row.split(" ");
                let size_or_dir = dir_info.next().unwrap();
                let file_type;
                let file_size;
                if size_or_dir == "dir" {
                    file_type = FileType::DIR;
                    file_size = None;
                } else {
                    file_type = FileType::FILE;
                    file_size = Some(size_or_dir.parse::<u32>().unwrap());
                }
                let name = dir_info.next().unwrap();
                current_node.children.push(FileNode {
                    name: String::from(name),
                    file_type,
                    size: file_size,
                    children: Vec::new(),
                });
            }
        }
    }

    let all_dirs = root_node.get_all_directories();
    let occupied_space = root_node.get_total_size();
    let max_occupied = TOTAL_SPACE - NEEDED_SPACE;
    let need_to_free = occupied_space - max_occupied;
    // Min remumable chunk that meets criteria
    let mut min_removable_chunk = TOTAL_SPACE;
    for dir in all_dirs {
        let size = dir.get_total_size();
        if size > need_to_free && size < min_removable_chunk {
            min_removable_chunk = size;
        }
    }
    println!("{min_removable_chunk}");
}
