/// LeetCode #1878 - Get Biggest Three Rhombus Sums in a Grid
use std::collections::BTreeSet;

fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let m = grid.len();
    let n = grid[0].len();
    let mut s1 = vec![vec![0i64; n + 2]; m + 1];
    let mut s2 = vec![vec![0i64; n + 2]; m + 1];
    for i in 0..m {
        for j in 0..n {
            s1[i + 1][j + 1] = s1[i][j] + grid[i][j] as i64;
            s2[i + 1][j + 1] = s2[i][j + 2] + grid[i][j] as i64;
        }
    }
    let mut ss = BTreeSet::new();
    for i in 0..m {
        for j in 0..n {
            let l = (i.min(m - 1 - i)).min(j.min(n - 1 - j));
            ss.insert(grid[i][j]);
            for k in 1..=l {
                let ii = i + 1;
                let jj = j + 1;
                let a = s1[ii + k][jj] - s1[ii][jj - k];
                let b = s1[ii][jj + k] - s1[ii - k][jj];
                let c = s2[ii][jj - k] - s2[ii - k][jj];
                let d = s2[ii + k][jj] - s2[ii][jj + k];
                let sum = a + b + c + d - grid[i + k][j] as i64 + grid[i - k][j] as i64;
                ss.insert(sum as i32);
            }
            while ss.len() > 3 {
                if let Some(&smallest) = ss.iter().next() {
                    ss.remove(&smallest);
                }
            }
        }
    }
    ss.into_iter().rev().collect()
}

fn main() {
    let grid = vec![
        vec![3, 4, 5, 1, 3],
        vec![3, 3, 4, 2, 3],
        vec![20, 30, 200, 40, 10],
        vec![1, 5, 5, 4, 1],
        vec![4, 3, 2, 2, 5],
    ];
    println!("{:?}", get_biggest_three(grid));
}

#[cfg(test)]
mod tests {
    use super::get_biggest_three;

    #[test]
    fn example_one() {
        let grid = vec![
            vec![3, 4, 5, 1, 3],
            vec![3, 3, 4, 2, 3],
            vec![20, 30, 200, 40, 10],
            vec![1, 5, 5, 4, 1],
            vec![4, 3, 2, 2, 5],
        ];
        assert_eq!(get_biggest_three(grid), vec![228, 216, 211]);
    }
}
