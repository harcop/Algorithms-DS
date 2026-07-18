/// LeetCode #2475 - Number of Unequal Triplets in Array
use std::collections::HashMap;

fn unequal_triplets(nums: Vec<i32>) -> i32 {
    let mut count = HashMap::new();
    for num in &nums {
        *count.entry(*num).or_insert(0) += 1;
    }

    let mut answer = 0i32;
    let mut prev = 0i32;
    let mut next = nums.len() as i32;

    for freq in count.values() {
        next -= freq;
        answer += prev * freq * next;
        prev += freq;
    }

    answer
}

fn main() {
    println!("{}", unequal_triplets(vec![4, 4, 2, 4, 3]));
}

#[cfg(test)]
mod tests {
    use super::unequal_triplets;

    #[test]
    fn example_one() {
        assert_eq!(unequal_triplets(vec![4, 4, 2, 4, 3]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(unequal_triplets(vec![1, 1, 1, 1, 1]), 0);
    }
}
