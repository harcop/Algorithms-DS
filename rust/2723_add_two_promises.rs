/// LeetCode #2723 - Add Two Promises (JS problem; Rust Result sum analogue)
fn add_two_promises(a: Result<i32, String>, b: Result<i32, String>) -> Result<i32, String> {
    Ok(a? + b?)
}

fn main() {
    println!("{:?}", add_two_promises(Ok(2), Ok(5)));
}

#[cfg(test)]
mod tests {
    use super::add_two_promises;

    #[test]
    fn example_one() {
        assert_eq!(add_two_promises(Ok(2), Ok(5)), Ok(7));
    }

    #[test]
    fn example_two() {
        assert_eq!(add_two_promises(Ok(10), Ok(-12)), Ok(-2));
    }
}
