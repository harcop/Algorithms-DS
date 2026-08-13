/// LeetCode #3190 - Find Minimum Operations to Make All Elements Divisible by Three
fn minimum_operations(nums: Vec<i32>) -> i32 {
    nums.iter().filter(|&&x| x % 3 != 0).count() as i32
}

fn main() {
    println!("{}", minimum_operations(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::minimum_operations;

    #[test]
    fn example1() {
        assert_eq!(minimum_operations(vec![1, 2, 3, 4]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_operations(vec![3, 6, 9]), 0);
    }
}
