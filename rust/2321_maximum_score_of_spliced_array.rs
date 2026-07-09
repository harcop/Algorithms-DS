/// LeetCode #2321 - Maximum Score Of Spliced Array
fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let s1: i32 = nums1.iter().sum();
    let s2: i32 = nums2.iter().sum();

    fn best_gain(nums1: &[i32], nums2: &[i32]) -> i32 {
        let mut t = nums1[0] - nums2[0];
        let mut mx = t;
        for i in 1..nums1.len() {
            let v = nums1[i] - nums2[i];
            t = if t > 0 { t + v } else { v };
            mx = mx.max(t);
        }
        mx
    }

    (s2 + best_gain(&nums1, &nums2)).max(s1 + best_gain(&nums2, &nums1))
}

fn main() {
    println!(
        "{}",
        maximums_spliced_array(vec![60, 60, 60], vec![10, 90, 10])
    );
}

#[cfg(test)]
mod tests {
    use super::maximums_spliced_array;

    #[test]
    fn example_one() {
        assert_eq!(
            maximums_spliced_array(vec![60, 60, 60], vec![10, 90, 10]),
            210
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximums_spliced_array(vec![20, 40, 20, 70, 30], vec![50, 20, 50, 40, 20]),
            220
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(maximums_spliced_array(vec![7, 11, 13], vec![1, 1, 1]), 31);
    }
}
