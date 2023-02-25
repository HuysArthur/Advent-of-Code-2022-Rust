pub fn result_part1(input: &str) -> i32 {
    let mut x: i32 = 1;
    let mut cycle: i32 = 0;

    let mut sum_signal_strengths: i32 = 0;

    for line in input.split('\n') {
        if line.starts_with("addx") {
            cycle += 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                sum_signal_strengths += x*cycle;
            }

            let (_, value) = line.split_once("addx ").unwrap();
            cycle += 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                sum_signal_strengths += x*cycle;
            }

            x += value.parse::<i32>().unwrap();
        } else {
            cycle += 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                sum_signal_strengths += x*cycle;
            }
        }
    }
    sum_signal_strengths
}

pub fn result_part2(input: &str) -> String {
    let mut sprite: i32 = 1;
    let mut cycle: i32 = 0;

    let mut output: String = String::from("");

    for line in input.split('\n') {
        if line.starts_with("addx") {
            cycle += 1;
            if cycle != 1 && (cycle-1)%40 == 0 {
                output = format!("{}\n", output);
            }
            if [sprite-1, sprite, sprite+1].contains(&((cycle-1)%40)) {
                output = format!("{}#", output);
            } else {
                output = format!("{}.", output);
            }

            cycle += 1;
            let (_, value) = line.split_once("addx ").unwrap();
            if cycle != 1 && (cycle-1)%40 == 0 {
                output = format!("{}\n", output);
            }
            if [sprite-1, sprite, sprite+1].contains(&((cycle-1)%40)) {
                output = format!("{}#", output);
            } else {
                output = format!("{}.", output);
            }

            sprite += value.parse::<i32>().unwrap();
        } else {
            cycle += 1;
            if cycle != 1 && (cycle-1)%40 == 0 {
                output = format!("{}\n", output);
            }
            if [sprite-1, sprite, sprite+1].contains(&((cycle-1)%40)) {
                output = format!("{}#", output);
            } else {
                output = format!("{}.", output);
            }
        }
    }
    output
}