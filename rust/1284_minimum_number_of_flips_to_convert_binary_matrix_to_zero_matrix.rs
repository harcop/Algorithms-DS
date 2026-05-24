/// LeetCode #1284 - Minimum Number of Flips to Convert Binary Matrix to Zero Matrix
use std::collections::{HashSet, VecDeque};

fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut start = 0u32;
    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 1 {
                start |= 1u32 << (i * n + j);
            }
        }
    }
    if start == 0 {
        return 0;
    }
    let dirs: [(i32, i32); 5] = [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)];
    let toggle = |state: u32, i: usize, j: usize| -> u32 {
        let mut s = state;
        for (di, dj) in dirs {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                let p = (ni as usize) * n + (nj as usize);
                s ^= 1u32 << p;
            }
        }
        s
    };
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back((start, 0i32));
    seen.insert(start);
    while let Some((state, steps)) = q.pop_front() {
        for i in 0..m {
            for j in 0..n {
                let ns = toggle(state, i, j);
                if ns == 0 {
                    return steps + 1;
                }
                if seen.insert(ns) {
                    q.push_back((ns, steps + 1));
                }
            }
        }
    }
    -1
}

fn main() {
    println!("{}", min_flips(vec![vec![1, 1], vec![1, 1]]));
}

#[cfg(test)]
mod tests {
    use super::min_flips;

    #[test]
    fn example_one() {
        assert_eq!(min_flips(vec![vec![0, 0], vec![0, 0]]), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_flips(vec![vec![0, 0], vec![0, 1]]), -1);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_flips(vec![vec![1, 1], vec![1, 1]]), 2);
    }
}
