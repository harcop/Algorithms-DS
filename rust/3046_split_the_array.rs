/// LeetCode #3046 - Split the Array
use std::collections::HashMap;

fn is_possible_to_split(nums: Vec<i32>) -> bool {
    let mut freq = HashMap::new();
    for &x in &nums {
        let c = freq.entry(x).or_insert(0);
        *c += 1;
        if *c > 2 {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", is_possible_to_split(vec![1, 1, 2, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::is_possible_to_split;

    #[test]
    fn example1() {
        assert!(is_possible_to_split(vec![1, 1, 2, 2, 3, 4]));
    }

    #[test]
    fn example2() {
        assert!(!is_possible_to_split(vec![1, 1, 1, 1]));
    }
}
