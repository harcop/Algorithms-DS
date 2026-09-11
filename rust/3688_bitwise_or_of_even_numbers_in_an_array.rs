/// LeetCode #3688 - Bitwise OR of Even Numbers in an Array
fn even_number_bitwise_ors(nums: Vec<i32>) -> i32 {
    nums.into_iter().filter(|x| x % 2 == 0).fold(0, |a, b| a | b)
}

fn main() {
    println!("{}", even_number_bitwise_ors(vec![1, 2, 3, 4, 5, 6]));
}

#[cfg(test)]
mod tests {
    use super::even_number_bitwise_ors;

    #[test]
    fn example1() {
        assert_eq!(even_number_bitwise_ors(vec![1, 2, 3, 4, 5, 6]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(even_number_bitwise_ors(vec![7, 9, 11]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(even_number_bitwise_ors(vec![1, 8, 16]), 24);
    }
}
