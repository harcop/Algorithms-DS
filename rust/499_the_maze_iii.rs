/// LeetCode #499 - The Maze III
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn find_shortest_way(maze: Vec<Vec<i32>>, ball: Vec<i32>, hole: Vec<i32>) -> String {
    let m = maze.len();
    let n = maze[0].len();
    let (hx, hy) = (hole[0] as usize, hole[1] as usize);
    let dirs = [(1i32, 0, 'd'), (0, -1, 'l'), (0, 1, 'r'), (-1, 0, 'u')];
    let mut dist = vec![vec![i32::MAX; n]; m];
    let mut path = vec![vec![String::from("~"); n]; m];
    let (sx, sy) = (ball[0] as usize, ball[1] as usize);
    dist[sx][sy] = 0;
    path[sx][sy] = String::new();
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0i32, String::new(), sx, sy)));
    while let Some(Reverse((d, p, x, y))) = pq.pop() {
        if x == hx && y == hy {
            return p;
        }
        if d > dist[x][y] || (d == dist[x][y] && p > path[x][y]) {
            continue;
        }
        for &(dx, dy, ch) in &dirs {
            let mut nx = x as i32;
            let mut ny = y as i32;
            let mut steps = 0;
            while {
                let tx = nx + dx;
                let ty = ny + dy;
                tx >= 0
                    && ty >= 0
                    && (tx as usize) < m
                    && (ty as usize) < n
                    && maze[tx as usize][ty as usize] == 0
            } {
                nx += dx;
                ny += dy;
                steps += 1;
                if nx as usize == hx && ny as usize == hy {
                    break;
                }
            }
            if steps == 0 {
                continue;
            }
            let (ux, uy) = (nx as usize, ny as usize);
            let nd = d + steps;
            let mut np = p.clone();
            np.push(ch);
            if nd < dist[ux][uy] || (nd == dist[ux][uy] && np < path[ux][uy]) {
                dist[ux][uy] = nd;
                path[ux][uy] = np.clone();
                pq.push(Reverse((nd, np, ux, uy)));
            }
        }
    }
    "impossible".into()
}

fn main() {
    let maze = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 1],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1],
        vec![0, 1, 0, 0, 0],
    ];
    println!("{}", find_shortest_way(maze, vec![4, 3], vec![0, 1]));
}

#[cfg(test)]
mod tests {
    use super::find_shortest_way;

    fn maze() -> Vec<Vec<i32>> {
        vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1],
            vec![0, 1, 0, 0, 0],
        ]
    }

    #[test]
    fn example_one() {
        assert_eq!(
            find_shortest_way(maze(), vec![4, 3], vec![0, 1]),
            "lul"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_shortest_way(maze(), vec![4, 3], vec![3, 0]),
            "impossible"
        );
    }

    #[test]
    fn example_three() {
        let maze = vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1],
        ];
        assert_eq!(
            find_shortest_way(maze, vec![0, 4], vec![3, 5]),
            "dldr"
        );
    }
}
