/// LeetCode #3512 - Minimum Operations to Make Array Sum Divisible by K
fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    nums.iter().sum::<i32>() % k
}

fn main() {
    println!("{}", min_operations(vec![3, 9, 7], 5));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![3, 9, 7], 5), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![4, 1, 3], 4), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![3, 2], 6), 5);
    }
}
