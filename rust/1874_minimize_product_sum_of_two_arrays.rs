/// LeetCode #1874 - Minimize Product Sum of Two Arrays
fn min_product_sum(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
    nums1.sort_unstable();
    nums2.sort_unstable_by(|a, b| b.cmp(a));
    nums1
        .iter()
        .zip(nums2.iter())
        .map(|(&x, &y)| x * y)
        .sum()
}

fn main() {
    println!("{}", min_product_sum(vec![5, 3, 4, 2], vec![4, 2, 2, 5]));
}

#[cfg(test)]
mod tests {
    use super::min_product_sum;

    #[test]
    fn example_one() {
        assert_eq!(min_product_sum(vec![5, 3, 4, 2], vec![4, 2, 2, 5]), 40);
    }
}
