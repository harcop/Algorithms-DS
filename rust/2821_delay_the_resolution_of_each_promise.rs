/// LeetCode #2821 - Delay the Resolution of Each Promise (JS problem; Rust analogue)
fn delay_promise_resolution(times: Vec<i32>, ms: i32) -> Vec<i32> {
    times.into_iter().map(|t| t + ms).collect()
}

fn main() {
    println!("{:?}", delay_promise_resolution(vec![30], 50));
}

#[cfg(test)]
mod tests {
    use super::delay_promise_resolution;

    #[test]
    fn example_one() {
        assert_eq!(delay_promise_resolution(vec![30], 50), vec![80]);
    }

    #[test]
    fn example_two() {
        assert_eq!(delay_promise_resolution(vec![50, 80], 70), vec![120, 150]);
    }

    #[test]
    fn example_three() {
        assert_eq!(delay_promise_resolution(vec![20, 100], 30), vec![50, 130]);
    }
}
