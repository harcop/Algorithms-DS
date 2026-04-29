/// LeetCode #88 - Merge Sorted Array
fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &[i32], n: i32) {
    let mut i = (m - 1) as isize;
    let mut j = (n - 1) as isize;
    let mut k = (m + n - 1) as isize;

    while j >= 0 {
        if i >= 0 && nums1[i as usize] > nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        k -= 1;
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    merge(&mut nums1, 3, &[2, 5, 6], 3);
    println!("{nums1:?}");
}

#[cfg(test)]
mod tests {
    use super::merge;

    #[test]
    fn example_one() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        merge(&mut nums1, 3, &[2, 5, 6], 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn example_two() {
        let mut nums1 = vec![1];
        merge(&mut nums1, 1, &[], 0);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn example_three() {
        let mut nums1 = vec![0];
        merge(&mut nums1, 0, &[1], 1);
        assert_eq!(nums1, vec![1]);
    }
}
