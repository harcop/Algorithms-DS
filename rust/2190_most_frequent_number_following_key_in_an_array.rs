/// LeetCode #2190 - Most Frequent Number Following Key In an Array
use std::collections::HashMap;

fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
    let mut cnt = HashMap::new();
    let mut best_count = 0i32;
    let mut ans = 0i32;

    for w in nums.windows(2) {
        if w[0] == key {
            let c = cnt.entry(w[1]).or_insert(0);
            *c += 1;
            if *c > best_count {
                best_count = *c;
                ans = w[1];
            }
        }
    }

    ans
}

fn main() {
    println!("{}", most_frequent(vec![1, 100, 200, 1, 100], 1));
}

#[cfg(test)]
mod tests {
    use super::most_frequent;

    #[test]
    fn example_one() {
        assert_eq!(most_frequent(vec![1, 100, 200, 1, 100], 1), 100);
    }

    #[test]
    fn example_two() {
        assert_eq!(most_frequent(vec![2, 2, 2, 2, 3], 2), 2);
    }
}
