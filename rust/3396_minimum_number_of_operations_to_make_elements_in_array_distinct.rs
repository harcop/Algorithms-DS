/// LeetCode #3396 - Minimum Number of Operations to Make Elements in Array Distinct
fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut s = std::collections::HashSet::new();
    for i in (0..nums.len()).rev() {
        if !s.insert(nums[i]) {
            return (i / 3 + 1) as i32;
        }
    }
    0
}

fn main() {
    println!("{}", minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7]));
}

#[cfg(test)]
mod tests {
    use super::minimum_operations;

    #[test]
    fn example1() {
        assert_eq!(minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_operations(vec![4, 5, 6, 4, 4]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_operations(vec![6, 7, 8, 9]), 0);
    }
}
