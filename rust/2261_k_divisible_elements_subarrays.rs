/// LeetCode #2261 - K Divisible Elements Subarrays
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<i32, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
        }
    }
}

fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
    let mut root = TrieNode::new();
    let mut ans = 0;

    for i in 0..nums.len() {
        insert(&mut root, &nums, i, k, p, &mut ans);
    }

    ans
}

fn insert(node: &mut TrieNode, nums: &[i32], i: usize, k: i32, p: i32, ans: &mut i32) {
    if i == nums.len() || k - i32::from(nums[i] % p == 0) < 0 {
        return;
    }
    if !node.children.contains_key(&nums[i]) {
        node.children.insert(nums[i], TrieNode::new());
        *ans += 1;
    }
    let child = node.children.get_mut(&nums[i]).unwrap();
    insert(child, nums, i + 1, k - i32::from(nums[i] % p == 0), p, ans);
}

fn main() {
    println!("{}", count_distinct(vec![2, 3, 3, 2, 2, 2], 2, 2));
}

#[cfg(test)]
mod tests {
    use super::count_distinct;

    #[test]
    fn example_one() {
        assert_eq!(count_distinct(vec![2, 3, 3, 2, 2, 2], 2, 2), 11);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_distinct(vec![1, 2, 3, 4], 4, 1), 10);
    }
}
