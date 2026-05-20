/// LeetCode #1133 - Largest Unique Number
use std::collections::HashMap;

fn largest_number(nums: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for &x in &nums {
        *cnt.entry(x).or_insert(0) += 1;
    }
    cnt.into_iter()
        .filter(|(_, c)| *c == 1)
        .map(|(k, _)| k)
        .max()
        .unwrap_or(-1)
}

fn main() {
    println!("{}", largest_number(vec![5, 7, 3, 9, 4, 9, 8, 3, 5]));
}

#[cfg(test)]
mod tests {
    use super::largest_number;

    #[test]
    fn example_one() {
        assert_eq!(largest_number(vec![5, 7, 3, 9, 4, 9, 8, 3, 5]), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_number(vec![9, 9, 8, 8]), -1);
    }
}
