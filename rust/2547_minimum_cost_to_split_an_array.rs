/// LeetCode #2547 - Minimum Cost to Split an Array
fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
    const MAX: usize = 1001;
    let n = nums.len();
    let mut trimmed_length = vec![vec![0; n]; n];
    let mut dp = vec![i32::MAX / 2; n + 1];

    for i in 0..n {
        let mut length = 0;
        let mut count = [0i32; MAX];
        for j in i..n {
            count[nums[j] as usize] += 1;
            if count[nums[j] as usize] == 2 {
                length += 2;
            } else if count[nums[j] as usize] > 2 {
                length += 1;
            }
            trimmed_length[i][j] = length;
        }
    }

    dp[n] = 0;
    for i in (0..n).rev() {
        for j in i..n {
            dp[i] = dp[i].min(k + trimmed_length[i][j] + dp[j + 1]);
        }
    }
    dp[0]
}

fn main() {
    println!("{}", min_cost(vec![1, 2, 1, 2, 1, 3, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example_one() {
        assert_eq!(min_cost(vec![1, 2, 1, 2, 1, 3, 3], 2), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_cost(vec![1, 2, 1, 2, 1], 2), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_cost(vec![1, 2, 1, 2, 1], 5), 10);
    }
}
