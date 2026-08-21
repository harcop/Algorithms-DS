/// LeetCode #3346 - Maximum Frequency of an Element After Performing Operations I
use std::collections::HashMap;

fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut d: HashMap<i32, i32> = HashMap::new();
    for &x in &nums {
        *cnt.entry(x).or_insert(0) += 1;
        d.entry(x).or_insert(0);
        *d.entry(x - k).or_insert(0) += 1;
        *d.entry(x + k + 1).or_insert(0) -= 1;
    }
    let mut keys: Vec<i32> = d.keys().copied().collect();
    keys.sort_unstable();
    let mut ans = 0;
    let mut s = 0;
    for x in keys {
        s += d[&x];
        ans = ans.max(s.min(cnt.get(&x).copied().unwrap_or(0) + num_operations));
    }
    ans
}

fn main() {
    println!("{}", max_frequency(vec![1, 4, 5], 1, 2));
}

#[cfg(test)]
mod tests {
    use super::max_frequency;

    #[test]
    fn example1() {
        assert_eq!(max_frequency(vec![1, 4, 5], 1, 2), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(max_frequency(vec![5, 11, 20, 20], 5, 1), 2);
    }
}
