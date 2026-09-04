/// LeetCode #805 - Split Array With Same Average
use std::collections::HashSet;

fn split_array_same_average(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n < 2 {
        return false;
    }
    let total: i32 = nums.iter().sum();
    let mut dp: Vec<HashSet<i32>> = vec![HashSet::new(); n / 2 + 1];
    dp[0].insert(0);
    for &x in &nums {
        for k in (1..=n / 2).rev() {
            let prev: Vec<i32> = dp[k - 1].iter().copied().collect();
            for s in prev {
                dp[k].insert(s + x);
            }
        }
    }
    for k in 1..=n / 2 {
        if (total * k as i32) % n as i32 == 0 {
            let target = total * k as i32 / n as i32;
            if dp[k].contains(&target) {
                return true;
            }
        }
    }
    false
}

fn main() {
    println!("{}", split_array_same_average(vec![1, 2, 3, 4, 5, 6, 7, 8]));
}

#[cfg(test)]
mod tests {
    use super::split_array_same_average;

    #[test]
    fn example_one() {
        assert!(split_array_same_average(vec![1, 2, 3, 4, 5, 6, 7, 8]));
    }

    #[test]
    fn example_two() {
        assert!(!split_array_same_average(vec![3, 1]));
    }
}
