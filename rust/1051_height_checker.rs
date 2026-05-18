/// LeetCode #1051 - Height Checker
fn height_checker(heights: Vec<i32>) -> i32 {
    let mut sorted = heights.clone();
    sorted.sort_unstable();
    heights.iter().zip(&sorted).filter(|(a, b)| a != b).count() as i32
}

fn main() {
    println!("{}", height_checker(vec![1, 1, 4, 2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::height_checker;

    #[test]
    fn example_one() {
        assert_eq!(height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(height_checker(vec![5, 1, 2, 3, 4]), 5);
    }
}
