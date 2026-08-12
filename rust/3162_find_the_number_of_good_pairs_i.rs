/// LeetCode #3162 - Find the Number of Good Pairs I
fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    for &x in &nums1 {
        for &y in &nums2 {
            if x % (y * k) == 0 {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1));
}

#[cfg(test)]
mod tests {
    use super::number_of_pairs;

    #[test]
    fn example1() {
        assert_eq!(number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3), 2);
    }
}
