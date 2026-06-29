/// LeetCode #2152 - Minimum Number of Lines to Cover Points
fn minimum_lines(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    if n <= 2 {
        return 1;
    }

    let mut lines = vec![vec![0usize; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            let mut mask = 0usize;
            let x1 = points[i][0] as i64;
            let y1 = points[i][1] as i64;
            let x2 = points[j][0] as i64;
            let y2 = points[j][1] as i64;
            for (k, p) in points.iter().enumerate() {
                let x3 = p[0] as i64;
                let y3 = p[1] as i64;
                if (x2 - x1) * (y3 - y1) == (y2 - y1) * (x3 - x1) {
                    mask |= 1 << k;
                }
            }
            lines[i][j] = mask;
            lines[j][i] = mask;
        }
    }

    let full = (1usize << n) - 1;
    let mut memo = vec![-1i32; 1 << n];

    fn dfs(mask: usize, n: usize, full: usize, lines: &[Vec<usize>], memo: &mut [i32]) -> i32 {
        if mask == full {
            return 0;
        }
        if memo[mask] != -1 {
            return memo[mask];
        }

        let first = (0..n).find(|&i| (mask >> i) & 1 == 0).unwrap();
        let mut ans = 1 + dfs(mask | (1 << first), n, full, lines, memo);
        for j in first + 1..n {
            if (mask >> j) & 1 == 0 {
                ans = ans.min(1 + dfs(mask | lines[first][j], n, full, lines, memo));
            }
        }
        memo[mask] = ans;
        ans
    }

    dfs(0, n, full, &lines, &mut memo)
}

fn main() {
    println!(
        "{}",
        minimum_lines(vec![vec![0, 1], vec![2, 3], vec![4, 5], vec![4, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_lines;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_lines(vec![vec![0, 1], vec![2, 3], vec![4, 5], vec![4, 3]]),
            2
        );
    }

    #[test]
    fn all_collinear() {
        assert_eq!(minimum_lines(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 1);
    }
}
