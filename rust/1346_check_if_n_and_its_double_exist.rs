/// LeetCode #1346 - Check If N And Its Double Exist

use std::collections::HashSet;

fn check_if_exist(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    for &x in &nums {
        if seen.contains(&(2 * x)) || (x % 2 == 0 && seen.contains(&(x / 2))) {
            return true;
        }
        seen.insert(x);
    }
    false
}

fn main() {
    println!("{}", check_if_exist(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::check_if_exist;

    #[test]
    fn example_one() {
        assert!(check_if_exist(vec![1, 2, 3, 4]));
    }

    #[test]
    fn example_two() {
        assert!(!check_if_exist(vec![3, 1, 7, 11]));
    }
}
