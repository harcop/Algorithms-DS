/// LeetCode #3537 - Fill a Special Grid
fn special_grid(n: i32) -> Vec<Vec<i32>> {
    let m = 1usize << n;
    let mut ans = vec![vec![0; m]; m];
    let mut val = 0i32;
    fn dfs(x: usize, y: usize, k: usize, ans: &mut [Vec<i32>], val: &mut i32) {
        if k == 1 {
            ans[x][y] = *val;
            *val += 1;
            return;
        }
        let half = k / 2;
        dfs(x, y, half, ans, val);
        dfs(x + half, y, half, ans, val);
        dfs(x + half, y - half, half, ans, val);
        dfs(x, y - half, half, ans, val);
    }
    dfs(0, m - 1, m, &mut ans, &mut val);
    ans
}

fn main() {
    println!("{:?}", special_grid(1));
}

#[cfg(test)]
mod tests {
    use super::special_grid;

    #[test]
    fn example1() {
        assert_eq!(special_grid(0), vec![vec![0]]);
    }

    #[test]
    fn example2() {
        assert_eq!(special_grid(1), vec![vec![3, 0], vec![2, 1]]);
    }

    #[test]
    fn example3() {
        assert_eq!(
            special_grid(2),
            vec![
                vec![15, 12, 3, 0],
                vec![14, 13, 2, 1],
                vec![11, 8, 7, 4],
                vec![10, 9, 6, 5],
            ]
        );
    }
}
