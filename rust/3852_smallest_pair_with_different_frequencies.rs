/// LeetCode #3852 - Smallest Pair With Different Frequencies
use std::collections::HashMap;

fn min_distinct_freq_pair(nums: Vec<i32>) -> Vec<i32> {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut x = i32::MAX;
    for &v in &nums {
        *cnt.entry(v).or_insert(0) += 1;
        x = x.min(v);
    }
    let mut min_y = i32::MAX;
    for &y in cnt.keys() {
        if y < min_y && cnt[&x] != cnt[&y] {
            min_y = y;
        }
    }
    if min_y == i32::MAX {
        vec![-1, -1]
    } else {
        vec![x, min_y]
    }
}

fn main() {
    println!("{:?}", min_distinct_freq_pair(vec![1, 1, 2, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::min_distinct_freq_pair;

    #[test]
    fn example1() {
        assert_eq!(min_distinct_freq_pair(vec![1, 1, 2, 2, 3, 4]), vec![1, 3]);
    }

    #[test]
    fn example2() {
        assert_eq!(min_distinct_freq_pair(vec![1, 5]), vec![-1, -1]);
    }

    #[test]
    fn example3() {
        assert_eq!(min_distinct_freq_pair(vec![7]), vec![-1, -1]);
    }
}
