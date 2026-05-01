/// LeetCode #136 - Single Number
fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |a, b| a ^ b)
}

fn main() {
    println!("{}", single_number(vec![4, 1, 2, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::single_number;

    #[test]
    fn example_one() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(single_number(vec![1]), 1);
    }
}
