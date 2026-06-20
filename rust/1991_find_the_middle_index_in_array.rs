/// LeetCode #1991 - Find the Middle Index in Array
fn find_middle_index(nums: Vec<i32>) -> i32 {
    let mut left = 0i32;
    let mut right: i32 = nums.iter().sum();
    for (i, &x) in nums.iter().enumerate() {
        right -= x;
        if left == right {
            return i as i32;
        }
        left += x;
    }
    -1
}

fn main() {
    println!("{}", find_middle_index(vec![2, 3, -1, 8, 4]));
}

#[cfg(test)]
mod tests {
    use super::find_middle_index;

    #[test]
    fn example_one() {
        assert_eq!(find_middle_index(vec![2, 3, -1, 8, 4]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_middle_index(vec![1, -1, 4]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_middle_index(vec![2, 5]), -1);
    }
}
