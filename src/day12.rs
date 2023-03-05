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

pub fn result_part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = create_grid(input);

    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.to_owned() == 'S' {
                paths.push(vec![(x, y)]);
            }
        }
    }

    while !paths.is_empty() && !paths.iter().any(|path| grid[path.last().unwrap().1][path.last().unwrap().0] == 'E') {
        let cloned_paths = paths.clone();
        paths.clear();
        for path in cloned_paths {
            let (pivot_x, pivot_y) = path.last().unwrap();

            // Left
            if pivot_x.to_owned() != 0 &&
            !path.contains(&(pivot_x-1, pivot_y.to_owned())) &&
            ((grid[pivot_y.to_owned()][pivot_x-1] != 'E' && grid[pivot_y.to_owned()][pivot_x.to_owned()] as i32 - grid[pivot_y.to_owned()][pivot_x-1] as i32 >= -1) ||
            grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'z' && grid[pivot_y.to_owned()][pivot_x-1] == 'E' ||
            grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'S' && grid[pivot_y+1][pivot_x.to_owned()] == 'a') {
                let mut new_path:Vec<(usize, usize)> = path.clone();
                new_path.push((pivot_x-1, pivot_y.to_owned()));
                paths.push(new_path);
            }
            
            // Right
            if pivot_x + 1 < grid[0].len() &&
            !path.contains(&(pivot_x+1, pivot_y.to_owned())) &&
            ((grid[pivot_y.to_owned()][pivot_x+1] != 'E' && grid[pivot_y.to_owned()][pivot_x.to_owned()] as i32 - grid[pivot_y.to_owned()][pivot_x+1] as i32 >= -1) ||
            grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'z' && grid[pivot_y.to_owned()][pivot_x+1] == 'E' ||
            grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'S' && grid[pivot_y+1][pivot_x.to_owned()] == 'a') {
                let mut new_path:Vec<(usize, usize)> = path.clone();
                new_path.push((pivot_x+1, pivot_y.to_owned()));
                paths.push(new_path);
            }

            // Up
            if pivot_y.to_owned() != 0 &&
            !path.contains(&(pivot_x.to_owned(), pivot_y-1)) &&
            ((grid[pivot_y-1][pivot_x.to_owned()] != 'E' && grid[pivot_y.to_owned()][pivot_x.to_owned()] as i32 - grid[pivot_y-1][pivot_x.to_owned()] as i32 >= -1) ||
            grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'z' && grid[pivot_y-1][pivot_x.to_owned()] == 'E' ||
            grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'S' && grid[pivot_y+1][pivot_x.to_owned()] == 'a') {
                let mut new_path:Vec<(usize, usize)> = path.clone();
                new_path.push((pivot_x.to_owned(), pivot_y-1));
                paths.push(new_path);
            }
            
            // Down
            if pivot_y + 1 < grid.len() &&
            !path.contains(&(pivot_x.to_owned(), pivot_y+1)) &&
            ((grid[pivot_y+1][pivot_x.to_owned()] != 'E' && grid[pivot_y.to_owned()][pivot_x.to_owned()] as i32 - grid[pivot_y+1][pivot_x.to_owned()] as i32 >= -1) ||
            grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'z' && grid[pivot_y+1][pivot_x.to_owned()] == 'E' ||
            grid[pivot_y.to_owned()][pivot_x.to_owned()] == 'S' && grid[pivot_y+1][pivot_x.to_owned()] == 'a') {
                let mut new_path:Vec<(usize, usize)> = path.clone();
                new_path.push((pivot_x.to_owned(), pivot_y+1));
                paths.push(new_path);
            }
        }
        println!("{}", paths.iter().map(|path| path.len()).max().unwrap())
    }

    paths.iter().find(|path| grid[path.last().unwrap().1][path.last().unwrap().0] == 'E').unwrap().len() - 1
}