/// LeetCode #3131 - Find the Integer Added to Array I
fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    nums2.iter().copied().min().unwrap() - nums1.iter().copied().min().unwrap()
}

fn main() {
    println!("{}", added_integer(vec![2, 6, 4], vec![9, 7, 5]));
}

#[cfg(test)]
mod tests {
    use super::added_integer;

    #[test]
    fn example1() {
        assert_eq!(added_integer(vec![2, 6, 4], vec![9, 7, 5]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(added_integer(vec![10], vec![5]), -5);
    }

    #[test]
    fn example3() {
        assert_eq!(added_integer(vec![1, 1, 1, 1], vec![1, 1, 1, 1]), 0);
    }
}
