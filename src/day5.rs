pub fn result_part1(input: &str) -> String {
    let (crates_string, operations_string) = input.split_once("\n\n").unwrap();

    let crate_lines: Vec<Vec<char>> = crates_string
        .split('\n')
        .filter(|&s| s.ne(crates_string.split('\n').last().unwrap()))
        .map(|s| format!("{} ", s).chars().rev().collect())
        .rev()
        .collect();

    let amount_crates = crate_lines[0].len()/4;
    let mut crates: Vec<Vec<char>> = vec![vec![];amount_crates];

    for crate_line in crate_lines.iter() {
        for (index, chunk) in crate_line.chunks(4).enumerate() {
            let character = chunk[2];
            if character != ' ' {
                crates[amount_crates-index-1].push(character)
            }
        }
    }

    for operation_line in operations_string.split('\n').map(|s| s.trim()) {
        let (amount_str, rest) = operation_line[5..].split_once(" from ").unwrap();
        let (from_str, to_str) = rest.split_once(" to ").unwrap();

        let amount: i8 = amount_str.parse().unwrap();
        let from: usize = from_str.parse().unwrap();
        let to: usize = to_str.parse().unwrap();

        for _ in 0..amount {
            let x = crates[from-1].pop().unwrap();
            crates[to-1].push(x);
        }
    }

    crates.iter().map(|v| v.last().unwrap().to_string()).reduce(|acc, x| format!("{}{}", acc, x)).unwrap()
}

pub fn result_part2(input: &str) -> String {
    let (crates_string, operations_string) = input.split_once("\n\n").unwrap();

    let crate_lines: Vec<Vec<char>> = crates_string
        .split('\n')
        .filter(|&s| s.ne(crates_string.split('\n').last().unwrap()))
        .map(|s| format!("{} ", s).chars().rev().collect())
        .rev()
        .collect();

    let amount_crates = crate_lines[0].len()/4;
    let mut crates: Vec<Vec<char>> = vec![vec![];amount_crates];

    for crate_line in crate_lines.iter() {
        for (index, chunk) in crate_line.chunks(4).enumerate() {
            let character = chunk[2];
            if character != ' ' {
                crates[amount_crates-index-1].push(character)
            }
        }
    }

    for operation_line in operations_string.split('\n').map(|s| s.trim()) {
        let (amount_str, rest) = operation_line[5..].split_once(" from ").unwrap();
        let (from_str, to_str) = rest.split_once(" to ").unwrap();

        let amount: i8 = amount_str.parse().unwrap();
        let from: usize = from_str.parse().unwrap();
        let to: usize = to_str.parse().unwrap();

        let mut to_move: Vec<char> = Vec::new();
        for _ in 0..amount {
            let x = crates[from-1].pop().unwrap();
            to_move.push(x);
        }

        to_move.reverse();
        for c in to_move {
            crates[to-1].push(c);
        }
    }

    crates.iter().map(|v| v.last().unwrap().to_string()).reduce(|acc, x| format!("{}{}", acc, x)).unwrap()
}
