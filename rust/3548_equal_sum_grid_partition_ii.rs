/// LeetCode #3548 - Equal Sum Grid Partition II
use std::collections::HashMap;

fn check(g: &[Vec<i32>]) -> bool {
    let m = g.len();
    let n = g[0].len();
    let mut s1 = 0i64;
    let mut s2 = 0i64;
    let mut cnt1: HashMap<i64, i32> = HashMap::new();
    let mut cnt2: HashMap<i64, i32> = HashMap::new();
    for row in g {
        for &x in row {
            s2 += x as i64;
            *cnt2.entry(x as i64).or_insert(0) += 1;
        }
    }
    for i in 0..m - 1 {
        for &x in &g[i] {
            let x = x as i64;
            s1 += x;
            s2 -= x;
            *cnt1.entry(x).or_insert(0) += 1;
            *cnt2.entry(x).or_insert(0) -= 1;
        }
        if s1 == s2 {
            return true;
        }
        if s1 < s2 {
            let diff = s2 - s1;
            if cnt2.get(&diff).copied().unwrap_or(0) > 0
                && ((m - i - 1 > 1 && n > 1)
                    || (i == m - 2 && (g[i + 1][0] as i64 == diff || g[i + 1][n - 1] as i64 == diff))
                    || (n == 1 && (g[i + 1][0] as i64 == diff || g[m - 1][0] as i64 == diff)))
            {
                return true;
            }
        } else {
            let diff = s1 - s2;
            if cnt1.get(&diff).copied().unwrap_or(0) > 0
                && ((i + 1 > 1 && n > 1)
                    || (i == 0 && (g[0][0] as i64 == diff || g[0][n - 1] as i64 == diff))
                    || (n == 1 && (g[0][0] as i64 == diff || g[i][0] as i64 == diff)))
            {
                return true;
            }
        }
    }
    false
}

fn transpose(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut t = vec![vec![0; m]; n];
    for i in 0..m {
        for j in 0..n {
            t[j][i] = grid[i][j];
        }
    }
    t
}

fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
    let t = transpose(&grid);
    check(&grid) || check(&t)
}

fn main() {
    println!("{}", can_partition_grid(vec![vec![1, 4], vec![2, 3]]));
}

#[cfg(test)]
mod tests {
    use super::can_partition_grid;

    #[test]
    fn example1() {
        assert_eq!(can_partition_grid(vec![vec![1, 4], vec![2, 3]]), true);
    }

    #[test]
    fn example2() {
        assert_eq!(can_partition_grid(vec![vec![1, 2], vec![3, 4]]), true);
    }

    #[test]
    fn example3() {
        assert_eq!(can_partition_grid(vec![vec![1, 2, 4], vec![2, 3, 5]]), false);
    }

    #[test]
    fn example4() {
        assert_eq!(can_partition_grid(vec![vec![4, 1, 8], vec![3, 2, 6]]), false);
    }
}
