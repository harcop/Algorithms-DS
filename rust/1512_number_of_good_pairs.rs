/// LeetCode #1512 - Number Of Good Pairs
use std::collections::HashMap;

fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut ans = 0;
    for x in nums {
        ans += *cnt.get(&x).unwrap_or(&0);
        *cnt.entry(x).or_insert(0) += 1;
    }
    ans
}

fn main() {
    println!("{}", num_identical_pairs(vec![1, 2, 3, 1, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::num_identical_pairs;

    #[test]
    fn example_one() {
        assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6);
    }
}
