/// LeetCode #2258 - Escape the Spreading Fire
use std::collections::VecDeque;

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let max_minutes = m * n;
    let mut lo = -1i32;
    let mut hi = max_minutes as i32;

    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if can_stay_for(&grid, mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }

    if lo == max_minutes as i32 {
        1_000_000_000
    } else {
        lo
    }
}

fn spread_fire(grid: &[Vec<i32>], fire: &mut [Vec<bool>], queue: VecDeque<(usize, usize)>) -> VecDeque<(usize, usize)> {
    let m = grid.len();
    let n = grid[0].len();
    let mut next = VecDeque::new();

    for (i, j) in queue {
        for (dx, dy) in DIRS {
            let x = i as i32 + dx;
            let y = j as i32 + dy;
            if x < 0 || x == m as i32 || y < 0 || y == n as i32 {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            if !fire[x][y] && grid[x][y] == 0 {
                fire[x][y] = true;
                next.push_back((x, y));
            }
        }
    }

    next
}

fn can_stay_for(grid: &[Vec<i32>], wait: i32) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let mut fire = vec![vec![false; n]; m];
    let mut fire_queue = VecDeque::new();

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                fire[i][j] = true;
                fire_queue.push_back((i, j));
            }
        }
    }

    let mut remaining = wait;
    while remaining > 0 && !fire_queue.is_empty() {
        fire_queue = spread_fire(grid, &mut fire, fire_queue);
        remaining -= 1;
    }

    if fire[0][0] {
        return false;
    }

    let mut person_queue = VecDeque::from([(0usize, 0usize)]);
    let mut seen = vec![vec![false; n]; m];
    seen[0][0] = true;

    while !person_queue.is_empty() {
        for _ in 0..person_queue.len() {
            let (i, j) = person_queue.pop_front().unwrap();
            if fire[i][j] {
                continue;
            }
            for (dx, dy) in DIRS {
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if x < 0 || x == m as i32 || y < 0 || y == n as i32 {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if seen[x][y] || fire[x][y] || grid[x][y] != 0 {
                    continue;
                }
                if x == m - 1 && y == n - 1 {
                    return true;
                }
                seen[x][y] = true;
                person_queue.push_back((x, y));
            }
        }
        fire_queue = spread_fire(grid, &mut fire, fire_queue);
    }

    false
}

fn main() {
    println!(
        "{}",
        maximum_minutes(vec![
            vec![0, 2, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 2, 2, 1, 0],
            vec![0, 2, 0, 0, 1, 2, 0],
            vec![0, 0, 2, 2, 2, 0, 2],
            vec![0, 0, 0, 0, 0, 0, 0]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_minutes;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_minutes(vec![
                vec![0, 2, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 2, 2, 1, 0],
                vec![0, 2, 0, 0, 1, 2, 0],
                vec![0, 0, 2, 2, 2, 0, 2],
                vec![0, 0, 0, 0, 0, 0, 0]
            ]),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_minutes(vec![vec![0, 0, 0], vec![0, 1, 2], vec![0, 2, 0]]), -1);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_minutes(vec![vec![0, 0, 0], vec![2, 2, 0], vec![1, 2, 0]]), 1_000_000_000);
    }
}
