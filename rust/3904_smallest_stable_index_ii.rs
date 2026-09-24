/// LeetCode #3904 - Smallest Stable Index II
fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut right = vec![0i32; n];
    right[n - 1] = nums[n - 1];
    for i in (0..n - 1).rev() {
        right[i] = right[i + 1].min(nums[i]);
    }
    let mut left = 0i32;
    for i in 0..n {
        left = left.max(nums[i]);
        if left - right[i] <= k {
            return i as i32;
        }
    }
    -1
}

fn main() {
    println!("{}", first_stable_index(vec![5, 0, 1, 4], 3));
}

#[cfg(test)]
mod tests {
    use super::first_stable_index;

    #[test]
    fn example1() {
        assert_eq!(first_stable_index(vec![5, 0, 1, 4], 3), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(first_stable_index(vec![3, 2, 1], 1), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(first_stable_index(vec![0], 0), 0);
    }
}
