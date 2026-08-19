/// LeetCode #3287 - Find the Maximum Sequence Value of Array
fn max_value(nums: Vec<i32>, k: i32) -> i32 {
    let m = 1 << 7;
    let n = nums.len();
    let k = k as usize;
    let mut f = vec![vec![vec![false; m]; k + 2]; n + 1];
    f[0][0][0] = true;
    for i in 0..n {
        for j in 0..=k {
            for x in 0..m {
                if !f[i][j][x] {
                    continue;
                }
                f[i + 1][j][x] = true;
                f[i + 1][j + 1][x | nums[i] as usize] = true;
            }
        }
    }
    let mut g = vec![vec![vec![false; m]; k + 2]; n + 1];
    g[n][0][0] = true;
    for i in (1..=n).rev() {
        for j in 0..=k {
            for y in 0..m {
                if !g[i][j][y] {
                    continue;
                }
                g[i - 1][j][y] = true;
                g[i - 1][j + 1][y | nums[i - 1] as usize] = true;
            }
        }
    }
    let mut ans = 0;
    for i in k..=n - k {
        for x in 0..m {
            if f[i][k][x] {
                for y in 0..m {
                    if g[i][k][y] {
                        ans = ans.max((x ^ y) as i32);
                    }
                }
            }
        }
    }
    ans
}

fn main() {
    println!("{}", max_value(vec![2, 6, 7], 1));
}

#[cfg(test)]
mod tests {
    use super::max_value;

    #[test]
    fn example1() {
        assert_eq!(max_value(vec![2, 6, 7], 1), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(max_value(vec![4, 2, 5, 6, 7], 2), 2);
    }
}
