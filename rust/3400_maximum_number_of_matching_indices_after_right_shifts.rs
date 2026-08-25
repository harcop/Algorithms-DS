/// LeetCode #3400 - Maximum Number of Matching Indices After Right Shifts
fn maximum_matching_indices(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let n = nums1.len();
    let mut ans = 0;
    for k in 0..n {
        let t = (0..n)
            .filter(|&i| nums1[(i + k) % n] == nums2[i])
            .count();
        ans = ans.max(t);
    }
    ans as i32
}

fn main() {
    println!(
        "{}",
        maximum_matching_indices(vec![3, 1, 2, 3, 1, 2], vec![1, 2, 3, 1, 2, 3])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_matching_indices;

    #[test]
    fn example1() {
        assert_eq!(
            maximum_matching_indices(vec![3, 1, 2, 3, 1, 2], vec![1, 2, 3, 1, 2, 3]),
            6
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            maximum_matching_indices(vec![1, 4, 2, 5, 3, 1], vec![2, 3, 1, 2, 4, 6]),
            3
        );
    }
}
