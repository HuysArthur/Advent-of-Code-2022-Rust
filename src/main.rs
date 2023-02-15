mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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

    let input_day5 = utils::read_file("input_files/day5.txt").unwrap();
    println!("Result day5 part1: {}", day5::result_part1(&input_day5));
    println!("Result day5 part2: {}", day5::result_part2(&input_day5));

    let input_day6 = utils::read_file("input_files/day6.txt").unwrap();
    println!("Result day6 part1: {}", day6::result_part1(&input_day6));
    println!("Result day6 part2: {}", day6::result_part2(&input_day6));

    let input_day7 = utils::read_file("input_files/day7.txt").unwrap();
    println!("Result day7 part1: {}", day7::result_part1(&input_day7));
    println!("Result day7 part2: {}", day7::result_part2(&input_day7));
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

#[cfg(test)]
mod tests_day5 {
    use crate::day5;

    const EXAMPLE_INPUT: &str =
    "    [D]    
    [N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2";

    #[test]
    fn part1_example() {
        assert_eq!(day5::result_part1(EXAMPLE_INPUT), "CMZ");
    }

    #[test]
    fn part2_example() {
        assert_eq!(day5::result_part2(EXAMPLE_INPUT),"MCD");
    }
}

#[cfg(test)]
mod tests_day6 {
    use crate::day6;

    const EXAMPLE_INPUT_1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE_INPUT_2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE_INPUT_3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE_INPUT_4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    const EXAMPLE_INPUT_5: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE_INPUT_6: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE_INPUT_7: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE_INPUT_8: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE_INPUT_9: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part1_example_1() {
        assert_eq!(day6::result_part1(EXAMPLE_INPUT_1), 5);
    }
    #[test]
    fn part1_example_2() {
        assert_eq!(day6::result_part1(EXAMPLE_INPUT_2), 6);
    }
    #[test]
    fn part1_example_3() {
        assert_eq!(day6::result_part1(EXAMPLE_INPUT_3), 10);
    }
    #[test]
    fn part1_example_4() {
        assert_eq!(day6::result_part1(EXAMPLE_INPUT_4), 11);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(day6::result_part2(EXAMPLE_INPUT_5), 19);
    }
    #[test]
    fn part2_example_2() {
        assert_eq!(day6::result_part2(EXAMPLE_INPUT_6), 23);
    }
    #[test]
    fn part2_example_3() {
        assert_eq!(day6::result_part2(EXAMPLE_INPUT_7), 23);
    }
    #[test]
    fn part2_example_4() {
        assert_eq!(day6::result_part2(EXAMPLE_INPUT_8), 29);
    }
    #[test]
    fn part2_example_5() {
        assert_eq!(day6::result_part2(EXAMPLE_INPUT_9), 26);
    }
}

#[cfg(test)]
mod tests_day7 {
    use crate::day7;

    const EXAMPLE_INPUT: &str = 
"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_example() {
        assert_eq!(day7::result_part1(EXAMPLE_INPUT), 95437);
    }

    #[test]
    fn part2_example() {
        assert_eq!(day7::result_part2(EXAMPLE_INPUT), 24933642);
    }
}