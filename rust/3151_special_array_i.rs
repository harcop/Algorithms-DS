/// LeetCode #3151 - Special Array I
fn is_array_special(nums: Vec<i32>) -> bool {
    nums.windows(2).all(|w| w[0] % 2 != w[1] % 2)
}

fn main() {
    println!("{}", is_array_special(vec![2, 1, 4]));
}

#[cfg(test)]
mod tests {
    use super::is_array_special;

    #[test]
    fn example1() {
        assert!(is_array_special(vec![1]));
    }

    #[test]
    fn example2() {
        assert!(is_array_special(vec![2, 1, 4]));
    }

    #[test]
    fn example3() {
        assert!(!is_array_special(vec![4, 3, 1, 6]));
    }
}
