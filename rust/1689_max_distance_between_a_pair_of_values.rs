/// LeetCode #1689 - Max Distance Between A Pair Of Values
fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut ans = 0i32;
    for i in 0..nums1.len() {
        for j in 0..nums2.len() {
            if nums1[i] <= nums2[j] {
                ans = ans.max(j as i32 - i as i32);
            }
        }
    }
    ans
}

fn main() {
    println!("{}", max_distance(vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5]));
}

#[cfg(test)]
mod tests {
    use super::max_distance;

    #[test]
    fn example_one() {
        assert_eq!(max_distance(vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_distance(vec![8, 1, 5, 2, 7], vec![7, 2, 5, 1, 7]), 3);
    }
}
