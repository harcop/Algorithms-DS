/// LeetCode #3366 - Minimum Array Sum
fn min_array_sum(nums: Vec<i32>, k: i32, op1: i32, op2: i32) -> i32 {
    let n = nums.len();
    let op1 = op1 as usize;
    let op2 = op2 as usize;
    let inf = i32::MAX / 4;
    let mut f = vec![vec![vec![inf; op2 + 1]; op1 + 1]; n + 1];
    f[0][0][0] = 0;
    for (i, &x) in nums.iter().enumerate() {
        let i = i + 1;
        for j in 0..=op1 {
            for t in 0..=op2 {
                f[i][j][t] = f[i - 1][j][t] + x;
                if j > 0 {
                    f[i][j][t] = f[i][j][t].min(f[i - 1][j - 1][t] + (x + 1) / 2);
                }
                if t > 0 && x >= k {
                    f[i][j][t] = f[i][j][t].min(f[i - 1][j][t - 1] + (x - k));
                }
                if j > 0 && t > 0 {
                    let y = (x + 1) / 2;
                    if y >= k {
                        f[i][j][t] = f[i][j][t].min(f[i - 1][j - 1][t - 1] + y - k);
                    }
                    if x >= k {
                        f[i][j][t] = f[i][j][t].min(f[i - 1][j - 1][t - 1] + (x - k + 1) / 2);
                    }
                }
            }
        }
    }
    let mut ans = inf;
    for j in 0..=op1 {
        for t in 0..=op2 {
            ans = ans.min(f[n][j][t]);
        }
    }
    ans
}

fn main() {
    println!("{}", min_array_sum(vec![2, 8, 3, 19, 3], 3, 1, 1));
}

#[cfg(test)]
mod tests {
    use super::min_array_sum;

    #[test]
    fn example1() {
        assert_eq!(min_array_sum(vec![2, 8, 3, 19, 3], 3, 1, 1), 23);
    }

    #[test]
    fn example2() {
        assert_eq!(min_array_sum(vec![2, 4, 3], 3, 2, 1), 3);
    }
}
