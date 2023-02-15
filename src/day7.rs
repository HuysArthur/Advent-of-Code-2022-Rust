use std::collections::HashMap;

fn create_file_structure(input: &str) -> HashMap<String, i32> {
    let mut file_structure: HashMap<String, i32> = HashMap::new();
    let mut current_position: String = String::from("");

    for line in input.split('\n') {
        if line.starts_with("$ cd ") {
            let (_, argument) = line.split_once("$ cd ").unwrap();

            if argument == "/" {
                current_position = "/".to_string();
            } else if argument == ".." {
                let mut list: Vec<&str> = current_position.split('/').collect();
                list.pop();
                let new_position = list.join("/");
                current_position = new_position;
            } else {
                let new_position: String = format!("{}/{}", current_position, argument);
                current_position = new_position;
            }
        } else if !(line.starts_with('$') || line.starts_with("dir ")) {
            let (size_str, _) = line.split_once(' ').unwrap();
            let size: i32 = size_str.parse().unwrap();

            let position_vector: Vec<&str> = current_position.split('/').collect();
            let mut position: String = position_vector[0].to_string();

            for index in 1..position_vector.len() {
                position = format!("{}/{}", position, position_vector[index]);

                file_structure.entry(position.clone()).and_modify(|e| *e += size).or_insert(size);
            }
        }
    }
    file_structure
}

pub fn result_part1(input: &str) -> i32 {
    let file_structure: HashMap<String, i32> = create_file_structure(input);

    file_structure.iter()
        .map(|(_, &x)| x)
        .filter(|&size| size <= 100_000)
        .sum()
}

pub fn result_part2(input: &str) -> i32 {
    let file_structure: HashMap<String, i32> = create_file_structure(input);
    
    let total_used_space: i32 = file_structure["/"];

    file_structure.iter()
        .map(|(_, &x)| x)
        .filter(|&size| size >= (total_used_space-40_000_000))
        .min()
        .unwrap()
}