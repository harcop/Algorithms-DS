/// LeetCode #2540 - Minimum Common Value
fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut i = 0usize;
    let mut j = 0usize;
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] == nums2[j] {
            return nums1[i];
        }
        if nums1[i] < nums2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    -1
}

fn main() {
    println!("{}", get_common(vec![1, 2, 3], vec![2, 4]));
}

#[cfg(test)]
mod tests {
    use super::get_common;

    #[test]
    fn example_one() {
        assert_eq!(get_common(vec![1, 2, 3], vec![2, 4]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]), 2);
    }
}
