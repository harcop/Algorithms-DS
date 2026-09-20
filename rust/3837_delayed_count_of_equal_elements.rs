/// LeetCode #3837 - Delayed Count of Equal Elements (premium)
use std::collections::HashMap;

fn delayed_count(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut ans = vec![0; n];
    let mut i = n as i32 - k as i32 - 2;
    while i >= 0 {
        let idx = i as usize;
        *cnt.entry(nums[idx + k + 1]).or_insert(0) += 1;
        ans[idx] = *cnt.get(&nums[idx]).unwrap_or(&0);
        i -= 1;
    }
    ans
}

fn main() {
    println!("{:?}", delayed_count(vec![1, 2, 1, 1], 1));
}

#[cfg(test)]
mod tests {
    use super::delayed_count;

    #[test]
    fn example1() {
        assert_eq!(delayed_count(vec![1, 2, 1, 1], 1), vec![2, 0, 0, 0]);
    }

    #[test]
    fn example2() {
        assert_eq!(delayed_count(vec![3, 1, 3, 1], 0), vec![1, 1, 0, 0]);
    }
}
