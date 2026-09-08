/// LeetCode #3641 - Longest Semi-Repeating Subarray
use std::collections::HashMap;

fn longest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut ans = 0usize;
    let mut cur = 0;
    let mut l = 0usize;
    for r in 0..nums.len() {
        let e = cnt.entry(nums[r]).or_insert(0);
        *e += 1;
        if *e == 2 {
            cur += 1;
        }
        while cur > k {
            let e = cnt.entry(nums[l]).or_insert(0);
            *e -= 1;
            if *e == 1 {
                cur -= 1;
            }
            l += 1;
        }
        ans = ans.max(r - l + 1);
    }
    ans as i32
}

fn main() {
    println!("{}", longest_subarray(vec![1, 2, 3, 1, 2, 3, 4], 2));
}

#[cfg(test)]
mod tests {
    use super::longest_subarray;

    #[test]
    fn example1() {
        assert_eq!(longest_subarray(vec![1, 2, 3, 1, 2, 3, 4], 2), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_subarray(vec![1, 1, 1, 1, 1], 4), 5);
    }

    #[test]
    fn example3() {
        assert_eq!(longest_subarray(vec![1, 1, 1, 1, 1], 0), 1);
    }
}
