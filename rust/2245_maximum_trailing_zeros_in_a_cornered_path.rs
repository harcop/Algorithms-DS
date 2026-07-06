/// LeetCode #2245 - Maximum Trailing Zeros in a Cornered Path
fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut left2 = vec![vec![0; n]; m];
    let mut left5 = vec![vec![0; n]; m];
    let mut top2 = vec![vec![0; n]; m];
    let mut top5 = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            left2[i][j] = factor_count(grid[i][j], 2);
            left5[i][j] = factor_count(grid[i][j], 5);
            if j > 0 {
                left2[i][j] += left2[i][j - 1];
                left5[i][j] += left5[i][j - 1];
            }
        }
    }

    for j in 0..n {
        for i in 0..m {
            top2[i][j] = factor_count(grid[i][j], 2);
            top5[i][j] = factor_count(grid[i][j], 5);
            if i > 0 {
                top2[i][j] += top2[i - 1][j];
                top5[i][j] += top5[i - 1][j];
            }
        }
    }

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            let curr2 = factor_count(grid[i][j], 2);
            let curr5 = factor_count(grid[i][j], 5);
            let l2 = left2[i][j];
            let l5 = left5[i][j];
            let r2 = left2[i][n - 1] - if j > 0 { left2[i][j - 1] } else { 0 };
            let r5 = left5[i][n - 1] - if j > 0 { left5[i][j - 1] } else { 0 };
            let t2 = top2[i][j];
            let t5 = top5[i][j];
            let d2 = top2[m - 1][j] - if i > 0 { top2[i - 1][j] } else { 0 };
            let d5 = top5[m - 1][j] - if i > 0 { top5[i - 1][j] } else { 0 };

            for (a2, a5) in [
                (l2 + t2 - curr2, l5 + t5 - curr5),
                (r2 + t2 - curr2, r5 + t5 - curr5),
                (l2 + d2 - curr2, l5 + d5 - curr5),
                (r2 + d2 - curr2, r5 + d5 - curr5),
            ] {
                ans = ans.max(a2.min(a5));
            }
        }
    }
    ans
}

fn factor_count(mut num: i32, factor: i32) -> i32 {
    let mut count = 0;
    while num % factor == 0 {
        num /= factor;
        count += 1;
    }
    count
}

fn main() {
    println!(
        "{}",
        max_trailing_zeros(vec![vec![23, 17, 19, 15, 16], vec![24, 5, 10, 1, 7]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_trailing_zeros;

    #[test]
    fn example_one() {
        assert_eq!(
            max_trailing_zeros(vec![vec![23, 17, 19, 15, 16], vec![24, 5, 10, 1, 7]]),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(max_trailing_zeros(vec![vec![4, 3, 2], vec![7, 6, 1], vec![8, 8, 8]]), 0);
    }
}
