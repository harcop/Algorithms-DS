/// LeetCode #2762 - Continuous Subarrays
use std::collections::BTreeMap;

fn continuous_subarrays(nums: Vec<i32>) -> i64 {
    let mut ans: i64 = 0;
    let mut freq: BTreeMap<i32, usize> = BTreeMap::new();
    let mut l = 0usize;
    for (r, &x) in nums.iter().enumerate() {
        *freq.entry(x).or_insert(0) += 1;
        while freq.keys().last().unwrap() - freq.keys().next().unwrap() > 2 {
            let lv = nums[l];
            let cnt = freq.get_mut(&lv).unwrap();
            if *cnt == 1 {
                freq.remove(&lv);
            } else {
                *cnt -= 1;
            }
            l += 1;
        }
        ans += (r - l + 1) as i64;
    }
    ans
}

fn main() {
    println!("{}", continuous_subarrays(vec![5, 4, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::continuous_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(continuous_subarrays(vec![5, 4, 2, 4]), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(continuous_subarrays(vec![1, 2, 3]), 6);
    }
}
