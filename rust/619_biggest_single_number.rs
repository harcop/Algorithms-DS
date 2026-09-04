/// LeetCode #619 - Biggest Single Number (SQL; Rust analogue)
use std::collections::HashMap;

fn biggest_single_number(nums: Vec<i32>) -> Option<i32> {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for n in nums {
        *cnt.entry(n).or_insert(0) += 1;
    }
    cnt.into_iter().filter(|(_, c)| *c == 1).map(|(n, _)| n).max()
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::biggest_single_number;

    #[test]
    fn example_one() {
        assert_eq!(biggest_single_number(vec![8, 8, 3, 3, 1, 4, 5, 6]), Some(6));
    }

    #[test]
    fn example_two() {
        assert_eq!(biggest_single_number(vec![8, 8, 7, 7, 3, 3, 2, 2]), None);
    }
}
