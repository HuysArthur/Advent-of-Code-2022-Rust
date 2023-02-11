pub fn priority(c: &char) -> i32 {
    if c.is_lowercase() {
        return *c as i32 - 96;
    } else {
        *c as i32 - 38
    }
}

pub fn result_part1(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.split('\n') {
        let (first_half, second_half): (Vec<(usize, char)>, Vec<(usize, char)>) = line
            .trim()
            .chars()
            .enumerate()
            .partition(|(i, _)| *i < line.trim().len() / 2);

        let first_compartment: Vec<char> = first_half.iter().map(|(_, c)| c).cloned().collect();
        let second_compartment: Vec<char> = second_half.iter().map(|(_, c)| c).cloned().collect();

        let character = first_compartment.iter()
            .find(|c| second_compartment.contains(c))
            .unwrap();

        sum += priority(character);
    }

    sum
}

pub fn result_part2(input: &str) -> i32 {
    let mut sum = 0;
    
    let lines: Vec<Vec<char>> = input.split('\n').map(|s| s.trim().chars().collect()).collect();
    for chunk in lines.chunks(3) {
        let character = chunk[0].iter()
            .find(|c| chunk[1].contains(c) && chunk[2].contains(c))
            .unwrap();

        sum += priority(character);
    }

    sum
}