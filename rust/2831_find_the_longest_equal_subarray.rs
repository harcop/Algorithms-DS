/// LeetCode #2831 - Find the Longest Equal Subarray
use std::collections::HashMap;

fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let mut groups: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        groups.entry(x).or_default().push(i);
    }
    let mut ans = 0;
    for ids in groups.values() {
        let mut l = 0usize;
        for r in 0..ids.len() {
            while ids[r] - ids[l] - (r - l) > k as usize {
                l += 1;
            }
            ans = ans.max(r - l + 1);
        }
    }
    ans as i32
}

fn main() {
    println!("{}", longest_equal_subarray(vec![1, 3, 2, 3, 1, 3], 3));
}

#[cfg(test)]
mod tests {
    use super::longest_equal_subarray;

    #[test]
    fn example_one() {
        assert_eq!(longest_equal_subarray(vec![1, 3, 2, 3, 1, 3], 3), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_equal_subarray(vec![1, 1, 2, 2, 1, 1], 2), 4);
    }
}
