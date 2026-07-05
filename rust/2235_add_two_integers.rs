/// LeetCode #2235 - Add Two Integers
fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn main() {
    println!("{}", sum(12, 5));
}

#[cfg(test)]
mod tests {
    use super::sum;

    #[test]
    fn example_one() {
        assert_eq!(sum(12, 5), 17);
    }

    #[test]
    fn example_two() {
        assert_eq!(sum(-10, 4), -6);
    }
}
