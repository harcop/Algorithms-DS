/// LeetCode #2934 - Minimum Operations to Maximize Last Elements in Arrays
fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    fn f(nums1: &[i32], nums2: &[i32], x: i32, y: i32) -> i32 {
        let mut cnt = 0;
        for i in 0..nums1.len() - 1 {
            let a = nums1[i];
            let b = nums2[i];
            if a <= x && b <= y {
                continue;
            }
            if !(a <= y && b <= x) {
                return -1;
            }
            cnt += 1;
        }
        cnt
    }

    let a = f(&nums1, &nums2, nums1[nums1.len() - 1], nums2[nums2.len() - 1]);
    let b = f(&nums1, &nums2, nums2[nums2.len() - 1], nums1[nums1.len() - 1]);
    let mut ans = i32::MAX;
    if a >= 0 {
        ans = ans.min(a);
    }
    if b >= 0 {
        ans = ans.min(b + 1);
    }
    if ans == i32::MAX {
        -1
    } else {
        ans
    }
}

fn main() {
    println!("{}", min_operations(vec![1, 2, 7], vec![4, 5, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example_one() {
        assert_eq!(min_operations(vec![1, 2, 7], vec![4, 5, 3]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_operations(vec![2, 3, 4, 5, 9], vec![8, 8, 4, 4, 4]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_operations(vec![1, 5, 4], vec![2, 5, 3]), -1);
    }
}
