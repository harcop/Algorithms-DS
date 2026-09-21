/// LeetCode #3843 - First Element with Unique Frequency
use std::collections::HashMap;

fn first_unique_freq(nums: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for &x in &nums {
        *cnt.entry(x).or_insert(0) += 1;
    }
    let mut freq: HashMap<i32, i32> = HashMap::new();
    for &v in cnt.values() {
        *freq.entry(v).or_insert(0) += 1;
    }
    for x in nums {
        if freq[&cnt[&x]] == 1 {
            return x;
        }
    }
    -1
}

fn main() {
    println!("{}", first_unique_freq(vec![20, 10, 30, 30]));
}

#[cfg(test)]
mod tests {
    use super::first_unique_freq;

    #[test]
    fn example1() {
        assert_eq!(first_unique_freq(vec![20, 10, 30, 30]), 30);
    }

    #[test]
    fn example2() {
        assert_eq!(first_unique_freq(vec![20, 20, 10, 30, 30, 30]), 20);
    }

    #[test]
    fn example3() {
        assert_eq!(first_unique_freq(vec![10, 10, 20, 20]), -1);
    }
}
