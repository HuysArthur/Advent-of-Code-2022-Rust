mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

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

    let input_day8 = utils::read_file("input_files/day8.txt").unwrap();
    println!("Result day8 part1: {}", day8::result_part1(&input_day8));
    println!("Result day8 part2: {}", day8::result_part2(&input_day8));

    let input_day9 = utils::read_file("input_files/day9.txt").unwrap();
    println!("Result day9 part1: {}", day9::result_part1(&input_day9));
    println!("Result day9 part2: {}", day9::result_part2(&input_day9));

    let input_day10 = utils::read_file("input_files/day10.txt").unwrap();
    println!("Result day10 part1: {}", day10::result_part1(&input_day10));
    println!("Result day10 part2: \n{}", day10::result_part2(&input_day10));

    let input_day11 = utils::read_file("input_files/day11.txt").unwrap();
    println!("Result day11 part1: {}", day11::result_part1(&input_day11));
    // println!("Result day10 part2: \n{}", day10::result_part2(&input_day10));
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

#[cfg(test)]
mod tests_day8 {
    use crate::day8;

    const EXAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_example() {
        assert_eq!(day8::result_part1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(day8::result_part2(EXAMPLE_INPUT), 8);
    }
}

#[cfg(test)]
mod tests_day9 {
    use crate::day9;

    const EXAMPLE_INPUT_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const EXAMPLE_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_example() {
        assert_eq!(day9::result_part1(EXAMPLE_INPUT_1), 13);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(day9::result_part2(EXAMPLE_INPUT_1), 1);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(day9::result_part2(EXAMPLE_INPUT_2), 36);
    }
}

#[cfg(test)]
mod tests_day10 {
    use crate::day10;

    const EXAMPLE_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_example() {
        assert_eq!(day10::result_part1(EXAMPLE_INPUT), 13140);
    }

    #[test]
    fn part2_example() {
        assert_eq!(day10::result_part2(EXAMPLE_INPUT), "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....".to_string())
    }

}

#[cfg(test)]
mod test_day11 {
    use crate::day11;

    const EXAMPLE_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn result_part1() {
        assert_eq!(day11::result_part1(EXAMPLE_INPUT), 10605);
    }

    /*
    #[test]
    fn result_part2() {
        assert_eq!(day11::result_part2(EXAMPLE_INPUT), 2713310158);
    }
    */
}