/// LeetCode #1955 - Count Number of Special Subsequences
const MOD: i64 = 1_000_000_007;

fn count_special_subsequences(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut f = vec![[0i64; 3]; n];
    f[0][0] = if nums[0] == 0 { 1 } else { 0 };
    for i in 1..n {
        match nums[i] {
            0 => {
                f[i][0] = (2 * f[i - 1][0] + 1) % MOD;
                f[i][1] = f[i - 1][1];
                f[i][2] = f[i - 1][2];
            }
            1 => {
                f[i][0] = f[i - 1][0];
                f[i][1] = (f[i - 1][0] + 2 * f[i - 1][1]) % MOD;
                f[i][2] = f[i - 1][2];
            }
            _ => {
                f[i][0] = f[i - 1][0];
                f[i][1] = f[i - 1][1];
                f[i][2] = (f[i - 1][1] + 2 * f[i - 1][2]) % MOD;
            }
        }
    }
    f[n - 1][2] as i32
}

fn main() {
    println!("{}", count_special_subsequences(vec![0, 1, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::count_special_subsequences;

    #[test]
    fn example_one() {
        assert_eq!(count_special_subsequences(vec![0, 1, 2, 2]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_special_subsequences(vec![2, 2, 0, 0]), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_special_subsequences(vec![0, 1, 2, 0, 1, 2]), 7);
    }
}
