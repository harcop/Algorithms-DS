/// LeetCode #2524 - Maximum Frequency Score of a Subarray
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn mod_pow(mut x: i64, mut n: i32) -> i64 {
    let mut ans = 1i64;
    x %= MOD;
    while n > 0 {
        if n % 2 == 1 {
            ans = ans * x % MOD;
        }
        x = x * x % MOD;
        n /= 2;
    }
    ans
}

fn max_frequency_score(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut count: HashMap<i32, i32> = HashMap::new();
    for &num in &nums[..k] {
        *count.entry(num).or_default() += 1;
    }

    let mut sum = 0i64;
    for (&num, &freq) in &count {
        sum = (sum + mod_pow(num as i64, freq)) % MOD;
    }
    let mut ans = sum;

    for i in k..nums.len() {
        let left_num = nums[i - k];
        let left_freq = count[&left_num];
        sum = (sum - mod_pow(left_num as i64, left_freq) + MOD) % MOD;
        if left_freq - 1 > 0 {
            *count.get_mut(&left_num).unwrap() -= 1;
            sum = (sum + mod_pow(left_num as i64, left_freq - 1)) % MOD;
        } else {
            count.remove(&left_num);
        }

        let right_num = nums[i];
        if let Some(&right_freq) = count.get(&right_num) {
            if right_freq > 0 {
                sum = (sum - mod_pow(right_num as i64, right_freq) + MOD) % MOD;
            }
            *count.get_mut(&right_num).unwrap() += 1;
            sum = (sum + mod_pow(right_num as i64, right_freq + 1)) % MOD;
        } else {
            count.insert(right_num, 1);
            sum = (sum + mod_pow(right_num as i64, 1)) % MOD;
        }

        ans = ans.max(sum);
    }

    ans as i32
}

fn main() {
    println!("{}", max_frequency_score(vec![1, 1, 1, 2, 1, 2], 3));
}

#[cfg(test)]
mod tests {
    use super::max_frequency_score;

    #[test]
    fn example_one() {
        assert_eq!(max_frequency_score(vec![1, 1, 1, 2, 1, 2], 3), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_frequency_score(vec![1, 1, 1, 1, 1, 1], 4), 1);
    }
}
