fn is_visible(grid: &Vec<Vec<i32>>, (x, y): (usize, usize)) -> bool {
    let height = grid[y][x];

    if 
    (grid[0..y].iter().map(|e| e[x]).filter(|&e| e >= height).count() == 0) ||
    (grid[y+1..grid.len()].iter().map(|e| e[x]).filter(|&e| e >= height).count() == 0) ||
    (grid[y][0..x].iter().filter(|&&e| e >= height).count() == 0) ||
    (grid[y][x+1..grid[0].len()].iter().filter(|&&e| e >= height).count() == 0) {
        return true;
    }
    false
}

pub fn result_part1(input: &str) -> i32 {
    let mut count: i32 = 0;

    let grid: Vec<Vec<i32>> = input.split('\n')
        .map(|s| s.chars().map(|e| e as i32 - 48).collect())
        .collect();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if is_visible(&grid, (x, y)) {
                count += 1;
            }
        }
    }
    count
}

fn calc_scenic_score(grid: &Vec<Vec<i32>>, (x, y): (usize, usize)) -> i32 {
    let height = grid[y][x];

    let up: Vec<i32> = grid[0..y].iter().map(|e| e[x]).rev().collect();
    let down: Vec<i32> = grid[y+1..grid.len()].iter().map(|e| e[x]).collect();
    let left: Vec<i32> = grid[y][0..x].iter().map(|&e| e).rev().collect();
    let right: Vec<i32> = grid[y][x+1..grid[0].len()].iter().map(|&e| e).collect();

    let mut scenic_up: i32 = 0;
    for e in up {
        scenic_up += 1;
        if e >= height {
            break;
        }
    }

    let mut scenic_down: i32 = 0;
    for e in down {
        scenic_down += 1;
        if e >= height {
            break;
        }
    }

    let mut scenic_left: i32 = 0;
    for e in left {
        scenic_left += 1;
        if e >= height {
            break;
        }
    }

    let mut scenic_right: i32 = 0;
    for e in right {
        scenic_right += 1;
        if e >= height {
            break;
        }
    }
    scenic_up * scenic_down * scenic_left * scenic_right
}

pub fn result_part2(input: &str) -> i32 {
    let grid: Vec<Vec<i32>> = input.split('\n')
        .map(|s| s.chars().map(|e| e as i32 - 48).collect())
        .collect();

    let mut scenic_scores: Vec<i32> = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            scenic_scores.push(calc_scenic_score(&grid, (x, y)));
        }
    }

    *scenic_scores.iter().max().unwrap()
}