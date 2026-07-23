/// LeetCode #2619 - Array Prototype Last (JS problem; Rust Vec analogue)
fn last(arr: &[i32]) -> i32 {
    arr.last().copied().unwrap_or(-1)
}

fn main() {
    println!("{}", last(&[1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::last;

    #[test]
    fn example_one() {
        assert_eq!(last(&[0, 0, 3]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(last(&[]), -1);
    }
}
