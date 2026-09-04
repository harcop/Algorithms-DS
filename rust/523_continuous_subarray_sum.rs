/// LeetCode #523 - Continuous Subarray Sum
use std::collections::HashMap;

fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let mut seen = HashMap::new();
    seen.insert(0, -1);
    let mut sum = 0i64;
    let k = k as i64;
    for (i, x) in nums.iter().enumerate() {
        sum += *x as i64;
        let rem = if k == 0 { sum } else { sum.rem_euclid(k) };
        if let Some(&j) = seen.get(&rem) {
            if i as i32 - j >= 2 {
                return true;
            }
        } else {
            seen.insert(rem, i as i32);
        }
    }
    false
}

fn main() {
    println!("{}", check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
}

#[cfg(test)]
mod tests {
    use super::check_subarray_sum;

    #[test]
    fn example_one() {
        assert!(check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
    }

    #[test]
    fn example_two() {
        assert!(check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
    }

    #[test]
    fn example_three() {
        assert!(!check_subarray_sum(vec![23, 2, 6, 4, 7], 13));
    }
}
