/// LeetCode #1589 - Maximum Sum Obtained Of Any Permutation
const MOD: i64 = 1_000_000_007;

fn max_sum_hungry(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let mut diff = vec![0i64; n + 1];
    for r in requests {
        diff[r[0] as usize] += 1;
        diff[r[1] as usize + 1] -= 1;
    }
    for i in 1..=n {
        diff[i] += diff[i - 1];
    }
    let mut a = nums;
    a.sort_unstable();
    diff[..n].sort_unstable();
    let mut ans = 0i64;
    for i in 0..n {
        ans = (ans + a[i] as i64 * diff[i]) % MOD;
    }
    ans as i32
}

fn main() {
    println!(
        "{}",
        max_sum_hungry(vec![1, 2, 3, 4, 5, 6], vec![vec![0, 3], vec![0, 5]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_sum_hungry;

    #[test]
    fn example_one() {
        assert_eq!(
            max_sum_hungry(vec![1, 2, 3, 4, 5, 6], vec![vec![0, 3], vec![0, 5]]),
            39
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_sum_hungry(
                vec![1, 2, 3, 4, 5, 10],
                vec![vec![0, 1], vec![0, 5]],
            ),
            40
        );
    }
}
