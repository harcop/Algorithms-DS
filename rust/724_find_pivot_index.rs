/// LeetCode #724 - Find Pivot Index
fn pivot_index(nums: Vec<i32>) -> i32 {
    let total: i32 = nums.iter().sum();
    let mut left = 0i32;
    for (i, &x) in nums.iter().enumerate() {
        if left == total - left - x {
            return i as i32;
        }
        left += x;
    }
    -1
}

fn main() {
    println!("{}", pivot_index(vec![1, 7, 3, 6, 5, 6]));
}

#[cfg(test)]
mod tests {
    use super::pivot_index;

    #[test]
    fn example_one() {
        assert_eq!(pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(pivot_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn example_three() {
        assert_eq!(pivot_index(vec![2, 1, -1]), 0);
    }
}
