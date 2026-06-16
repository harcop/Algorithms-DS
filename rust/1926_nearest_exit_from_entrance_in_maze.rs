/// LeetCode #1926 - Nearest Exit from Entrance in Maze
use std::collections::VecDeque;

fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    let m = maze.len();
    let n = maze[0].len();
    let (si, sj) = (entrance[0] as usize, entrance[1] as usize);
    maze[si][sj] = '+';
    let mut q = VecDeque::from([(si, sj)]);
    let mut ans = 0i32;

    while !q.is_empty() {
        ans += 1;
        for _ in 0..q.len() {
            let (i, j) = q.pop_front().unwrap();
            for (di, dj) in [(0i32, -1), (0, 1), (-1, 0), (1, 0)] {
                let x = i as i32 + di;
                let y = j as i32 + dj;
                if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if maze[x][y] == '.' {
                    if x == 0 || x == m - 1 || y == 0 || y == n - 1 {
                        return ans;
                    }
                    maze[x][y] = '+';
                    q.push_back((x, y));
                }
            }
        }
    }
    -1
}

fn main() {
    let maze = vec![
        vec!['+', '+', '.', '+'],
        vec!['.', '.', '.', '+'],
        vec!['+', '+', '+', '.'],
    ];
    println!("{}", nearest_exit(maze, vec![1, 2]));
}

#[cfg(test)]
mod tests {
    use super::nearest_exit;

    #[test]
    fn example_one() {
        let maze = vec![
            vec!['+', '+', '.', '+'],
            vec!['.', '.', '.', '+'],
            vec!['+', '+', '+', '.'],
        ];
        assert_eq!(nearest_exit(maze, vec![1, 2]), 1);
    }

    #[test]
    fn example_two() {
        let maze = vec![vec!['+', '+', '+'], vec!['.', '.', '.'], vec!['+', '+', '+']];
        assert_eq!(nearest_exit(maze, vec![1, 0]), 2);
    }
}
