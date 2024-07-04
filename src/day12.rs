use std::collections::VecDeque;

fn create_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for (y, row) in input.split('\n').map(|s| s.chars()).enumerate() {
        grid.push(Vec::new());
        for c in row {
            grid[y].push(c);
        }
    }
    grid
}

fn calc_manh_distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

pub fn result_part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = create_grid(input);

    let mut paths: VecDeque<Vec<(usize, usize)>> = VecDeque::new();

    let mut end_position: (usize, usize) = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.to_owned() == 'E' {
                end_position = (x, y);
            }
        }
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.to_owned() == 'S' {
                paths.push_back(vec![(x, y)]);
            }
        }
    }

    while !paths.is_empty() && !paths.iter().any(|path| grid[path.last().unwrap().1][path.last().unwrap().0] == 'E') {
        let path = paths.pop_front().unwrap();
        //println!("{:?}", path);
        
        let (pivot_x, pivot_y) = path.last().unwrap();

        // Left
        if pivot_x.to_owned() != 0 &&
        !path.contains(&(pivot_x-1, pivot_y.to_owned())) &&
        ((grid[pivot_y.to_owned()][pivot_x-1] != 'E' && grid[pivot_y.to_owned()][pivot_x.to_owned()] as i32 - grid[pivot_y.to_owned()][pivot_x-1] as i32 >= -1) ||
        grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'z' && grid[pivot_y.to_owned()][pivot_x-1] == 'E' ||
        grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'S' && grid[pivot_y+1][pivot_x.to_owned()] == 'a') {
            let mut new_path:Vec<(usize, usize)> = path.clone();
            new_path.push((pivot_x-1, pivot_y.to_owned()));
            paths.push_back(new_path);
        }
        
        // Right
        if pivot_x + 1 < grid[0].len() &&
        !path.contains(&(pivot_x+1, pivot_y.to_owned())) &&
        ((grid[pivot_y.to_owned()][pivot_x+1] != 'E' && grid[pivot_y.to_owned()][pivot_x.to_owned()] as i32 - grid[pivot_y.to_owned()][pivot_x+1] as i32 >= -1) ||
        grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'z' && grid[pivot_y.to_owned()][pivot_x+1] == 'E' ||
        grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'S' && grid[pivot_y+1][pivot_x.to_owned()] == 'a') {
            let mut new_path:Vec<(usize, usize)> = path.clone();
            new_path.push((pivot_x+1, pivot_y.to_owned()));
            paths.push_back(new_path);
        }
        // Up
        if pivot_y.to_owned() != 0 &&
        !path.contains(&(pivot_x.to_owned(), pivot_y-1)) &&
        ((grid[pivot_y-1][pivot_x.to_owned()] != 'E' && grid[pivot_y.to_owned()][pivot_x.to_owned()] as i32 - grid[pivot_y-1][pivot_x.to_owned()] as i32 >= -1) ||
        grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'z' && grid[pivot_y-1][pivot_x.to_owned()] == 'E' ||
        grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'S' && grid[pivot_y+1][pivot_x.to_owned()] == 'a') {
            let mut new_path:Vec<(usize, usize)> = path.clone();
            new_path.push((pivot_x.to_owned(), pivot_y-1));
            paths.push_back(new_path);
        }
        
        // Down
        if pivot_y + 1 < grid.len() &&
        !path.contains(&(pivot_x.to_owned(), pivot_y+1)) &&
        ((grid[pivot_y+1][pivot_x.to_owned()] != 'E' && grid[pivot_y.to_owned()][pivot_x.to_owned()] as i32 - grid[pivot_y+1][pivot_x.to_owned()] as i32 >= -1) ||
        grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'z' && grid[pivot_y+1][pivot_x.to_owned()] == 'E' ||
        grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'S' && grid[pivot_y+1][pivot_x.to_owned()] == 'a') {
            let mut new_path:Vec<(usize, usize)> = path.clone();
            new_path.push((pivot_x.to_owned(), pivot_y+1));
            paths.push_back(new_path);
        }
        paths.make_contiguous().sort_by(|path1, path2| 
            (calc_manh_distance(path1.last().unwrap().to_owned(), end_position) + path1.len()-1).partial_cmp(
                &(calc_manh_distance(path2.last().unwrap().to_owned(), end_position) + path2.len()-1)).unwrap());
    }
    paths.iter().find(|path| grid[path.last().unwrap().1][path.last().unwrap().0] == 'E').unwrap().len() - 1
}