/// LeetCode #1855 - Maximum Distance Between a Pair of Values
fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut nums2 = nums2;
    nums2.reverse();
    let mut ans = 0i32;
    for (i, &v) in nums1.iter().enumerate() {
        let pos = nums2.partition_point(|&x| x < v);
        let j = nums2.len() as i32 - pos as i32 - 1;
        ans = ans.max(j - i as i32);
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_distance(vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5])
    );
}

#[cfg(test)]
mod tests {
    use super::max_distance;

    #[test]
    fn example_one() {
        assert_eq!(
            max_distance(vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5]),
            2
        );
    }
}
