/// LeetCode #675 - Cut Off Trees for Golf Event
use std::collections::VecDeque;

fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
    let m = forest.len();
    let n = forest[0].len();
    let mut trees: Vec<(i32, usize, usize)> = vec![];
    for i in 0..m {
        for j in 0..n {
            if forest[i][j] > 1 {
                trees.push((forest[i][j], i, j));
            }
        }
    }
    trees.sort();

    fn bfs(forest: &[Vec<i32>], sr: usize, sc: usize, tr: usize, tc: usize) -> i32 {
        let m = forest.len();
        let n = forest[0].len();
        if sr == tr && sc == tc {
            return 0;
        }
        let mut visited = vec![vec![false; n]; m];
        let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();
        q.push_back((sr, sc, 0));
        visited[sr][sc] = true;
        while let Some((r, c, d)) = q.pop_front() {
            for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nr >= m as i32 || nc < 0 || nc >= n as i32 {
                    continue;
                }
                let nr = nr as usize;
                let nc = nc as usize;
                if visited[nr][nc] || forest[nr][nc] == 0 {
                    continue;
                }
                if nr == tr && nc == tc {
                    return d + 1;
                }
                visited[nr][nc] = true;
                q.push_back((nr, nc, d + 1));
            }
        }
        -1
    }

    let mut sr = 0usize;
    let mut sc = 0usize;
    let mut total = 0i32;
    for (_, tr, tc) in trees {
        let d = bfs(&forest, sr, sc, tr, tc);
        if d == -1 {
            return -1;
        }
        total += d;
        sr = tr;
        sc = tc;
    }
    total
}

fn main() {
    println!(
        "{}",
        cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 4], vec![7, 6, 5]])
    );
}

#[cfg(test)]
mod tests {
    use super::cut_off_tree;

    #[test]
    fn example_one() {
        assert_eq!(
            cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 4], vec![7, 6, 5]]),
            6
        );
    }

    #[test]
    fn unreachable() {
        assert_eq!(
            cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 0], vec![7, 6, 5]]),
            -1
        );
    }
}
