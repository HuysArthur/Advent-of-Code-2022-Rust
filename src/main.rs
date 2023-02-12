mod utils;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let input_day1 = utils::read_file("input_files/day1.txt").unwrap();
    println!("Result day1 part1: {}", day1::result_part2(&input_day1));
    println!("Result day1 part2: {}", day1::result_part1(&input_day1));
    
    let input_day2 = utils::read_file("input_files/day2.txt").unwrap();
    println!("Result day2 part1: {}", day2::result_part1(&input_day2));
    println!("Result day2 part2: {}", day2::result_part2(&input_day2));
    
    let input_day3 = utils::read_file("input_files/day3.txt").unwrap();
    println!("Result day3 part1: {}", day3::result_part1(&input_day3));
    println!("Result day3 part2: {}", day3::result_part2(&input_day3));
    
    let input_day4 = utils::read_file("input_files/day4.txt").unwrap();
    println!("Result day4 part1: {}", day4::result_part1(&input_day4));
    println!("Result day4 part2: {}", day4::result_part2(&input_day4));
}

#[cfg(test)]
mod tests_day1 {
    use crate::day1;

    const EXAMPLE_INPUT: &str = 
    "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000";

    #[test]
    fn part1_example() {

        assert_eq!(day1::result_part1(EXAMPLE_INPUT), 24000);
    }

    #[test]
    fn part2_example() {
        assert_eq!(day1::result_part2(EXAMPLE_INPUT), 45000);
    }
}

#[cfg(test)]
mod tests_day2 {
    use crate::day2;

    const EXAMPLE_INPUT: &str =
    "A Y
    B X
    C Z";

    #[test]
    fn part1_example() {
        assert_eq!(day2::result_part1(EXAMPLE_INPUT), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(day2::result_part2(EXAMPLE_INPUT), 12);
    }
}

#[cfg(test)]
mod tests_day3 {
    use crate::day3;

    const EXAMPLE_INPUT: &str =
    "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_example() {
        assert_eq!(day3::result_part1(EXAMPLE_INPUT), 157);
    }

    #[test]
    fn part2_example() {
        assert_eq!(day3::result_part2(EXAMPLE_INPUT), 70);
    }
}

#[cfg(test)]
mod tests_day4 {
    use crate::day4;

    const EXAMPLE_INPUT: &str =
    "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

    #[test]
    fn part1_example() {
        assert_eq!(day4::result_part1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(day4::result_part2(EXAMPLE_INPUT), 4);
    }
}