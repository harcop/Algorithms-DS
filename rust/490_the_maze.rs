/// LeetCode #490 - The Maze
use std::collections::VecDeque;

fn has_path(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
    let m = maze.len();
    let n = maze[0].len();
    let dest = (destination[0] as usize, destination[1] as usize);
    let mut vis = vec![vec![false; n]; m];
    let mut q = VecDeque::new();
    let (sx, sy) = (start[0] as usize, start[1] as usize);
    q.push_back((sx, sy));
    vis[sx][sy] = true;
    let dirs = [(0i32, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some((x, y)) = q.pop_front() {
        if (x, y) == dest {
            return true;
        }
        for (dx, dy) in dirs {
            let mut nx = x as i32;
            let mut ny = y as i32;
            while {
                let tx = nx + dx;
                let ty = ny + dy;
                tx >= 0
                    && ty >= 0
                    && tx < m as i32
                    && ty < n as i32
                    && maze[tx as usize][ty as usize] == 0
            } {
                nx += dx;
                ny += dy;
            }
            let (ux, uy) = (nx as usize, ny as usize);
            if !vis[ux][uy] {
                vis[ux][uy] = true;
                q.push_back((ux, uy));
            }
        }
    }
    false
}

fn main() {
    let maze = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 0, 1, 1],
        vec![0, 0, 0, 0, 0],
    ];
    println!("{}", has_path(maze, vec![0, 4], vec![4, 4]));
}

#[cfg(test)]
mod tests {
    use super::has_path;

    fn maze() -> Vec<Vec<i32>> {
        vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![1, 1, 0, 1, 1],
            vec![0, 0, 0, 0, 0],
        ]
    }

    #[test]
    fn example_one() {
        assert!(has_path(maze(), vec![0, 4], vec![4, 4]));
    }

    #[test]
    fn example_two() {
        assert!(!has_path(maze(), vec![0, 4], vec![3, 2]));
    }

    #[test]
    fn example_three() {
        let maze = vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1],
            vec![0, 1, 0, 0, 0],
        ];
        assert!(!has_path(maze, vec![4, 3], vec![0, 1]));
    }
}
