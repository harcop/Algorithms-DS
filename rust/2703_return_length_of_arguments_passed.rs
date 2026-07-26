/// LeetCode #2703 - Return Length of Arguments Passed (JS problem; Rust analogue)
fn arguments_length<T>(args: &[T]) -> usize {
    args.len()
}

fn main() {
    println!("{}", arguments_length(&[5]));
}

#[cfg(test)]
mod tests {
    use super::arguments_length;

    #[test]
    fn example_one() {
        assert_eq!(arguments_length(&[5]), 1);
    }

    #[test]
    fn example_two() {
        let args: Vec<Option<i32>> = vec![Some(0), None, Some(3)];
        assert_eq!(arguments_length(&args), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(arguments_length(&[1, 2, 3]), 3);
    }
}
