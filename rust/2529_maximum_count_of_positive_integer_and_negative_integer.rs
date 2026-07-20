/// LeetCode #2529 - Maximum Count of Positive Integer and Negative Integer
fn maximum_count(nums: Vec<i32>) -> i32 {
    let pos = nums.iter().filter(|&&x| x > 0).count();
    let neg = nums.iter().filter(|&&x| x < 0).count();
    pos.max(neg) as i32
}

fn main() {
    println!("{}", maximum_count(vec![-2, -1, -1, 1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::maximum_count;

    #[test]
    fn example_one() {
        assert_eq!(maximum_count(vec![-2, -1, -1, 1, 2, 3]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_count(vec![5, 20, 66, 1314]), 4);
    }
}
