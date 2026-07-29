/// LeetCode #2778 - Sum of Squares of Special Elements
fn sum_of_squares(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    nums.iter()
        .enumerate()
        .filter(|(i, _)| n % (i + 1) == 0)
        .map(|(_, &x)| x * x)
        .sum()
}

fn main() {
    println!("{}", sum_of_squares(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::sum_of_squares;

    #[test]
    fn example_one() {
        assert_eq!(sum_of_squares(vec![1, 2, 3, 4]), 21);
    }

    #[test]
    fn example_two() {
        assert_eq!(sum_of_squares(vec![2, 7, 1, 19, 18, 3]), 63);
    }
}
