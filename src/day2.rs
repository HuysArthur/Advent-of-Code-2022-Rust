#[derive(PartialEq)]
enum Move {
    Scissors,
    Paper,
    Rock
}

#[derive(PartialEq)]
enum MatchResult {
    Win,
    Lose,
    Draw
}

pub fn result_part1(input: &str) -> i32 {

    fn map_code_to_move(code: char) -> Move {
        match code {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            'X' => Move::Rock,
            'Y' => Move::Paper,
            _ => Move::Scissors
        }
    }

    fn result_match(move_p1: &Move, move_p2: &Move) -> MatchResult {
        if move_p1 == move_p2 {
            MatchResult::Draw
        } else if 
            (*move_p1 == Move::Rock && *move_p2 == Move::Scissors) || 
            (*move_p1 == Move::Paper && *move_p2 == Move::Rock) || 
            (*move_p1 == Move::Scissors && *move_p2 == Move::Paper) {
            MatchResult::Win
        } else {
            MatchResult::Lose
        }
    }

    let mut score = 0;

    for line in input.split('\n') {
        let vec: Vec<char> = line.chars().filter(|e| *e != ' ').collect();
        let opp_move = map_code_to_move(vec[0]);
        let own_move = map_code_to_move(vec[1]);
        let result = result_match(&own_move, &opp_move);
        score += match own_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        };
        score += match result {
            MatchResult::Win => 6,
            MatchResult::Draw => 3,
            MatchResult::Lose => 0,
        };
    }
    score
}

pub fn result_part2(input: &str) -> i32 {

    fn map_code_to_move(code: char) -> Move {
        match code {
            'A' => Move::Rock,
            'B' => Move::Paper,
            _ => Move::Scissors
        }
    }

    fn map_code_to_result(code: char) -> MatchResult {
        match code {
            'X' => MatchResult::Lose,
            'Y' => MatchResult::Draw,
            _ => MatchResult::Win
        }
    }

    fn calc_move(opp_move: Move, result: &MatchResult) -> Move {
        if *result == MatchResult::Win {
            match opp_move {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock
            }
        } else if *result == MatchResult::Lose {
            match opp_move {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper
            }
        } else {
            opp_move
        }
    }

    let mut score = 0;

    for line in input.split('\n') {
        let vec: Vec<char> = line.chars().filter(|e| *e != ' ').collect();
        let opp_move = map_code_to_move(vec[0]);
        let result = map_code_to_result(vec[1]);
        let own_move = calc_move(opp_move, &result);
        score += match own_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        };
        score += match result {
            MatchResult::Win => 6,
            MatchResult::Draw => 3,
            MatchResult::Lose => 0,
        };
    }
    score
}