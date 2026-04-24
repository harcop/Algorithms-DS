/// LeetCode #33 - Search in Rotated Sorted Array
fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0i32;
    let mut right = nums.len() as i32 - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        let m = nums[mid as usize];
        if m == target {
            return mid;
        }

        if nums[left as usize] <= m {
            if nums[left as usize] <= target && target < m {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else if m < target && target <= nums[right as usize] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    -1
}

fn main() {
    println!("{}", search(vec![4, 5, 6, 7, 0, 1, 2], 0));
}

#[cfg(test)]
mod tests {
    use super::search;

    #[test]
    fn example_one() {
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn example_three() {
        assert_eq!(search(vec![1], 0), -1);
    }
}
