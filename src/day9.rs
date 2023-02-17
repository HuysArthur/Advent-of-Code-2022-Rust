fn move_next_knot((hx, hy): (i32, i32), (kx, ky): (i32, i32)) -> (i32, i32) {
    if (((hx-kx).abs().pow(2) + (hy-ky).abs().pow(2)) as f64).sqrt() < 2.0 {
        return (kx, ky);
    } else if hx == kx {
        if hy > ky {
            return (kx, ky+1);
        } else {
            return (kx, ky-1);
        }
    } else if hy == ky {
        if hx > kx {
            return (kx+1, ky);
        } else {
            return (kx-1, ky);
        }
    } else if hx > kx {
        if hy > ky {
            return (kx+1, ky+1);
        } else {
            return (kx+1, ky-1);
        }
    } else {
        if hy > ky {
            return (kx-1, ky+1);
        } else {
            return (kx-1, ky-1);
        }
    }
}

pub fn result_part1(input: &str) -> usize {
    let instructions: Vec<(&str, i32)> = input.split('\n')
        .map(|s| s.split_once(' ').unwrap())
        .map(|(direction, steps)| (direction, steps.parse().unwrap()))
        .collect();

    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    let mut tail_visited: Vec<(i32, i32)> = Vec::new();

    for (direction, steps) in instructions {
        for _ in 0..steps {
            if direction == "U" {
                head = (head.0, head.1 + 1);
            } else if direction == "D" {
                head = (head.0, head.1 - 1);
            } else if direction == "L" {
                head = (head.0 - 1, head.1);
            } else if direction == "R" {
                head = (head.0 + 1, head.1);
            }

            tail = move_next_knot(head, tail);
            if !tail_visited.contains(&tail) {
                tail_visited.push(tail.clone());
            }
        }
    }

    tail_visited.len()
}

pub fn result_part2(input: &str) -> usize {
    let instructions: Vec<(&str, i32)> = input.split('\n')
        .map(|s| s.split_once(' ').unwrap())
        .map(|(direction, steps)| (direction, steps.parse().unwrap()))
        .collect();

    let mut head: (i32, i32) = (0, 0);
    let mut knots: Vec<(i32, i32)> = Vec::new();
    for _ in 0..9 {
        knots.push((0, 0));
    }

    let mut tail_visited: Vec<(i32, i32)> = Vec::new();

    for (direction, steps) in instructions {
        for _ in 0..steps {
            if direction == "U" {
                head = (head.0, head.1 + 1);
            } else if direction == "D" {
                head = (head.0, head.1 - 1);
            } else if direction == "L" {
                head = (head.0 - 1, head.1);
            } else if direction == "R" {
                head = (head.0 + 1, head.1);
            }

            knots[0] = move_next_knot(head, knots[0]);
            for index in 1..knots.len() {
                knots[index] = move_next_knot(knots[index-1], knots[index]);
            }

            if !tail_visited.contains(knots.last().unwrap()) {
                tail_visited.push(knots.last().unwrap().clone());
            }
        }
    }

    tail_visited.len()
}