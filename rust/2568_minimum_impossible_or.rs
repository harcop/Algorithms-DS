/// LeetCode #2568 - Minimum Impossible OR
use std::collections::HashSet;

fn min_impossible_or(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = nums.into_iter().collect();
    let mut ans = 1;
    while set.contains(&ans) {
        ans <<= 1;
    }
    ans
}

fn main() {
    println!("{}", min_impossible_or(vec![2, 1]));
}

#[cfg(test)]
mod tests {
    use super::min_impossible_or;

    #[test]
    fn example_one() {
        assert_eq!(min_impossible_or(vec![2, 1]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_impossible_or(vec![5, 3, 2]), 1);
    }
}
