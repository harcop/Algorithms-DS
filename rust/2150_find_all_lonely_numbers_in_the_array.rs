/// LeetCode #2150 - Find All Lonely Numbers in the Array
use std::collections::HashMap;

fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
    let mut cnt = HashMap::new();
    for &x in &nums {
        *cnt.entry(x).or_insert(0) += 1;
    }

    nums.into_iter()
        .filter(|&x| cnt[&x] == 1 && !cnt.contains_key(&(x - 1)) && !cnt.contains_key(&(x + 1)))
        .collect()
}

fn main() {
    println!("{:?}", find_lonely(vec![10, 6, 5, 8]));
}

#[cfg(test)]
mod tests {
    use super::find_lonely;

    #[test]
    fn example_one() {
        assert_eq!(find_lonely(vec![10, 6, 5, 8]), vec![10, 8]);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_lonely(vec![1, 3, 5, 3]), vec![1, 5]);
    }
}
