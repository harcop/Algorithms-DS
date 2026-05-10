/// LeetCode #594 - Longest Harmonious Subsequence
use std::collections::HashMap;

fn find_lhs(nums: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for x in nums {
        *m.entry(x).or_insert(0) += 1;
    }
    let mut best = 0i32;
    for (&k, &c) in &m {
        if let Some(&d) = m.get(&(k + 1)) {
            best = best.max(c + d);
        }
    }
    best
}

fn main() {
    println!("{}", find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]));
}

#[cfg(test)]
mod tests {
    use super::find_lhs;

    #[test]
    fn example_one() {
        assert_eq!(find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
    }
}
