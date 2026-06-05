/// LeetCode #1730 - Shortest Path to Get Food
use std::collections::VecDeque;

fn get_food(mut grid: Vec<Vec<String>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut q = VecDeque::new();
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == "*" {
                q.push_back((i, j));
                break;
            }
        }
        if !q.is_empty() {
            break;
        }
    }
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut ans = 0;
    while !q.is_empty() {
        ans += 1;
        for _ in 0..q.len() {
            let (i, j) = q.pop_front().unwrap();
            for (di, dj) in dirs {
                let x = i as i32 + di;
                let y = j as i32 + dj;
                if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                match grid[x][y].as_str() {
                    "#" => return ans,
                    "O" => {
                        grid[x][y] = "X".into();
                        q.push_back((x, y));
                    }
                    _ => {}
                }
            }
        }
    }
    -1
}

fn main() {
    println!(
        "{}",
        get_food(vec![
            vec!["X".into(), "X".into(), "X".into(), "X".into(), "X".into(), "X".into()],
            vec!["X".into(), "*".into(), "O".into(), "O".into(), "O".into(), "X".into()],
            vec!["X".into(), "O".into(), "O".into(), "#".into(), "O".into(), "X".into()],
            vec!["X".into(), "X".into(), "X".into(), "X".into(), "X".into(), "X".into()],
        ])
    );
}
#[cfg(test)]
mod tests {
    use super::get_food;
    #[test]
    fn example_one() {
        assert_eq!(
            get_food(vec![
                vec!["X".into(), "X".into(), "X".into(), "X".into(), "X".into(), "X".into()],
                vec!["X".into(), "*".into(), "O".into(), "O".into(), "O".into(), "X".into()],
                vec!["X".into(), "O".into(), "O".into(), "#".into(), "O".into(), "X".into()],
                vec!["X".into(), "X".into(), "X".into(), "X".into(), "X".into(), "X".into()],
            ]),
            3
        );
    }
    #[test]
    fn example_two() {
        assert_eq!(
            get_food(vec![
                vec!["X".into(), "X".into(), "X".into(), "X".into(), "X".into()],
                vec!["X".into(), "*".into(), "X".into(), "O".into(), "X".into()],
                vec!["X".into(), "O".into(), "X".into(), "#".into(), "X".into()],
                vec!["X".into(), "X".into(), "X".into(), "X".into(), "X".into()],
            ]),
            -1
        );
    }
    #[test]
    fn example_three() {
        assert_eq!(
            get_food(vec![
                vec!["X".into(), "X".into(), "X".into(), "X".into(), "X".into(), "X".into(), "X".into(), "X".into()],
                vec!["X".into(), "*".into(), "O".into(), "X".into(), "O".into(), "#".into(), "O".into(), "X".into()],
                vec!["X".into(), "O".into(), "O".into(), "X".into(), "O".into(), "O".into(), "X".into(), "X".into()],
                vec!["X".into(), "O".into(), "O".into(), "O".into(), "O".into(), "#".into(), "O".into(), "X".into()],
                vec!["X".into(), "X".into(), "X".into(), "X".into(), "X".into(), "X".into(), "X".into(), "X".into()],
            ]),
            6
        );
    }
}
