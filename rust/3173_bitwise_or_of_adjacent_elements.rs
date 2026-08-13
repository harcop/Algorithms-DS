/// LeetCode #3173 - Bitwise OR of Adjacent Elements
fn or_array(nums: Vec<i32>) -> Vec<i32> {
    nums.windows(2).map(|w| w[0] | w[1]).collect()
}

fn main() {
    println!("{:?}", or_array(vec![1, 3, 7, 15]));
}

#[cfg(test)]
mod tests {
    use super::or_array;

    #[test]
    fn example1() {
        assert_eq!(or_array(vec![1, 3, 7, 15]), vec![3, 7, 15]);
    }

    #[test]
    fn example2() {
        assert_eq!(or_array(vec![8, 4, 2]), vec![12, 6]);
    }

    #[test]
    fn example3() {
        assert_eq!(or_array(vec![5, 4, 9, 11]), vec![5, 13, 11]);
    }
}
