use std::cmp::max;

fn make_list(input: &str) -> Vec<i32>{
    let mut list: Vec<i32> = vec![0];
    let mut index:usize = 0;

    for line in input.split('\n') {
        if line.trim().is_empty() {
            index += 1;
            list.push(0);
        } else {
            list[index] += line.trim().parse::<i32>().unwrap();
        }
    }
    list
}

pub fn result_part1(input: &str) -> i32 {
    *make_list(input)
        .iter()
        .reduce(|acc, e| max(acc, e))
        .unwrap()
}

pub fn result_part2(input: &str) -> i32 {
    let mut sorted_list = make_list(input);
    sorted_list.sort_by(|a, b| b.cmp(a));

    sorted_list.iter().take(3).sum()
}