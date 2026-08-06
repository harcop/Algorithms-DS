/// LeetCode #3043 - Find the Length of the Longest Common Prefix
use std::collections::HashSet;

fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut prefixes = HashSet::new();
    for &x in &arr1 {
        let mut n = x;
        while n > 0 {
            prefixes.insert(n);
            n /= 10;
        }
    }

    let mut max_match = 0;
    for &x in &arr2 {
        let mut n = x;
        while n > 0 && !prefixes.contains(&n) {
            n /= 10;
        }
        if n > max_match {
            max_match = n;
        }
    }

    if max_match == 0 {
        0
    } else {
        max_match.to_string().len() as i32
    }
}

fn main() {
    println!("{}", longest_common_prefix(vec![1, 10, 100], vec![1000]));
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;

    #[test]
    fn example1() {
        assert_eq!(longest_common_prefix(vec![1, 10, 100], vec![1000]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_common_prefix(vec![1, 2, 3], vec![4, 4, 4]), 0);
    }
}
