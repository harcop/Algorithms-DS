/// LeetCode #35 - Search Insert Position
fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0usize;
    let mut right = nums.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left as i32
}

fn main() {
    println!("{}", search_insert(vec![1, 3, 5, 6], 5));
}

#[cfg(test)]
mod tests {
    use super::search_insert;

    #[test]
    fn example_one() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
