fn covert_to_lower_upper_bound(string: &str) -> (i8, i8) {
    let (lower_bound_str, upper_bound_str) = string.split_once('-').unwrap();
    let lower_bound: i8 = lower_bound_str.parse().unwrap();
    let upper_bound: i8 = upper_bound_str.parse().unwrap();

    (lower_bound, upper_bound)
}

pub fn result_part1(input: &str) -> i32 {
    let mut count: i32 = 0;

    let lines: Vec<&str> = input.split('\n').map(|s| s.trim()).collect();
    for line in lines {
        let (first_elf, second_elf) = line.split_once(',').unwrap();
        let (first_assignment_first_elf, second_assignment_first_elf) = covert_to_lower_upper_bound(first_elf);
        let (first_assignment_second_elf, second_assignment_second_elf) = covert_to_lower_upper_bound(second_elf);

        if (first_assignment_first_elf >= first_assignment_second_elf && second_assignment_first_elf <= second_assignment_second_elf) ||
           (first_assignment_second_elf >= first_assignment_first_elf && second_assignment_second_elf <= second_assignment_first_elf) {
            count += 1
        }
    }
    count
}

pub fn result_part2(input: &str) -> i32 {
    let mut count: i32 = 0;

    let lines: Vec<&str> = input.split('\n').map(|s| s.trim()).collect();
    for line in lines {
        let (first_elf, second_elf) = line.split_once(',').unwrap();
        let (first_assignment_first_elf, second_assignment_first_elf) = covert_to_lower_upper_bound(first_elf);
        let (first_assignment_second_elf, second_assignment_second_elf) = covert_to_lower_upper_bound(second_elf);

        let assignments_first_elf: Vec<i8> = (first_assignment_first_elf..second_assignment_first_elf+1).collect();
        let assignments_second_elf: Vec<i8> = (first_assignment_second_elf..second_assignment_second_elf+1).collect();

        let amount_overlapping = assignments_first_elf.iter()
            .filter(|&a| assignments_second_elf.contains(a))
            .count();
        
        if amount_overlapping > 0 {
            count += 1;
        }
    }
    count
}