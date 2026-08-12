/// LeetCode #3158 - Find the XOR of Numbers Which Appear Twice
use std::collections::HashMap;

fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    let mut cnt = HashMap::new();
    for x in nums {
        *cnt.entry(x).or_insert(0) += 1;
    }
    cnt.into_iter()
        .filter(|&(_, v)| v == 2)
        .map(|(x, _)| x)
        .fold(0, |a, b| a ^ b)
}

fn main() {
    println!("{}", duplicate_numbers_xor(vec![1, 2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::duplicate_numbers_xor;

    #[test]
    fn example1() {
        assert_eq!(duplicate_numbers_xor(vec![1, 2, 1, 3]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(duplicate_numbers_xor(vec![1, 2, 3]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(duplicate_numbers_xor(vec![1, 2, 2, 1]), 3);
    }
}
