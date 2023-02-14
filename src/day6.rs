fn find_start_n_distinct_sequence(input: &str, n: usize) -> usize {
    let mut index: usize = 0;
    while index < input.len() - (n-1) {
        let (_, chars) = input.split_at(index);
        let last_four: Vec<char> = chars.chars().take(n).collect();

        if last_four.iter().filter(|&c| last_four.iter().filter(|&e| c == e).count() == 1).count() == n {
            break;
        }
        index += 1
    }
    index
}

pub fn result_part1(input: &str) -> usize {
    find_start_n_distinct_sequence(input, 4) + 4
}

pub fn result_part2(input: &str) -> usize {
    find_start_n_distinct_sequence(input, 14) + 14
}