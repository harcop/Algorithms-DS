/// LeetCode #2442 - Count Number of Distinct Integers After Reverse Operations
use std::collections::HashSet;

fn count_distinct_integers(nums: Vec<i32>) -> i32 {
    fn reverse(mut num: i32) -> i32 {
        let mut reversed = 0;
        while num > 0 {
            reversed = reversed * 10 + num % 10;
            num /= 10;
        }
        reversed
    }

    let mut values: HashSet<i32> = nums.iter().copied().collect();
    for num in nums {
        values.insert(reverse(num));
    }

    values.len() as i32
}

fn main() {
    println!("{}", count_distinct_integers(vec![1, 13, 10, 12, 31]));
}

#[cfg(test)]
mod tests {
    use super::count_distinct_integers;

    #[test]
    fn example_one() {
        assert_eq!(count_distinct_integers(vec![1, 13, 10, 12, 31]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_distinct_integers(vec![2, 2, 2]), 1);
    }
}
