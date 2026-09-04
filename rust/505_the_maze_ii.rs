/// LeetCode #505 - The Maze II
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn shortest_distance(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> i32 {
    let m = maze.len();
    let n = maze[0].len();
    let dest = (destination[0] as usize, destination[1] as usize);
    let mut dist = vec![vec![i32::MAX; n]; m];
    let mut pq = BinaryHeap::new();
    let (sx, sy) = (start[0] as usize, start[1] as usize);
    dist[sx][sy] = 0;
    pq.push(Reverse((0i32, sx, sy)));
    let dirs = [(0i32, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some(Reverse((d, x, y))) = pq.pop() {
        if (x, y) == dest {
            return d;
        }
        if d > dist[x][y] {
            continue;
        }
        for (dx, dy) in dirs {
            let mut nx = x as i32;
            let mut ny = y as i32;
            let mut steps = 0;
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
                steps += 1;
            }
            let (ux, uy) = (nx as usize, ny as usize);
            let nd = d + steps;
            if nd < dist[ux][uy] {
                dist[ux][uy] = nd;
                pq.push(Reverse((nd, ux, uy)));
            }
        }
    }
    -1
}

fn main() {
    let maze = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 0, 1, 1],
        vec![0, 0, 0, 0, 0],
    ];
    println!("{}", shortest_distance(maze, vec![0, 4], vec![4, 4]));
}

#[cfg(test)]
mod tests {
    use super::shortest_distance;

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
        assert_eq!(shortest_distance(maze(), vec![0, 4], vec![4, 4]), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(shortest_distance(maze(), vec![0, 4], vec![3, 2]), -1);
    }
}
