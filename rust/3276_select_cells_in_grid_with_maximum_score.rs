/// LeetCode #3276 - Select Cells in Grid With Maximum Score
use std::collections::{HashMap, HashSet};

fn max_score(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let mut g: HashMap<i32, HashSet<usize>> = HashMap::new();
    let mut mx = 0;
    for (i, row) in grid.iter().enumerate() {
        for &x in row {
            g.entry(x).or_default().insert(i);
            mx = mx.max(x);
        }
    }
    let mut f = vec![vec![0; 1 << m]; mx as usize + 1];
    for i in 1..=mx as usize {
        for j in 0..(1 << m) {
            f[i][j] = f[i - 1][j];
            if let Some(rows) = g.get(&(i as i32)) {
                for &k in rows {
                    if (j >> k) & 1 == 1 {
                        f[i][j] = f[i][j].max(f[i - 1][j ^ (1 << k)] + i as i32);
                    }
                }
            }
        }
    }
    *f[mx as usize].iter().max().unwrap()
}

fn main() {
    println!(
        "{}",
        max_score(vec![vec![1, 2, 3], vec![4, 3, 2], vec![1, 1, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example1() {
        assert_eq!(
            max_score(vec![vec![1, 2, 3], vec![4, 3, 2], vec![1, 1, 1]]),
            8
        );
    }

    #[test]
    fn example2() {
        assert_eq!(max_score(vec![vec![8, 7, 6], vec![8, 3, 2]]), 15);
    }
}
