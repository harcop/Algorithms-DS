/// LeetCode #2637 - Promise Time Limit (JS problem; simulated timeout analogue)
#[derive(Debug, PartialEq, Eq)]
enum LimitResult {
    Resolved(i32),
    Rejected(&'static str),
}

/// `fn_ms` is how long the async fn takes; `outcome` is Ok(value) or Err(message).
/// If `fn_ms > t`, reject with "Time Limit Exceeded" before the fn finishes.
fn time_limit(fn_ms: i32, outcome: Result<i32, &'static str>, t: i32) -> LimitResult {
    match outcome {
        Err(e) if fn_ms == 0 => LimitResult::Rejected(e),
        _ if fn_ms > t => LimitResult::Rejected("Time Limit Exceeded"),
        Ok(v) => LimitResult::Resolved(v),
        Err(e) => LimitResult::Rejected(e),
    }
}

fn main() {
    println!("{:?}", time_limit(100, Ok(25), 50));
}

#[cfg(test)]
mod tests {
    use super::{time_limit, LimitResult};

    #[test]
    fn example_one() {
        assert_eq!(
            time_limit(100, Ok(25), 50),
            LimitResult::Rejected("Time Limit Exceeded")
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(time_limit(100, Ok(25), 150), LimitResult::Resolved(25));
    }

    #[test]
    fn example_three() {
        assert_eq!(time_limit(120, Ok(15), 150), LimitResult::Resolved(15));
    }

    #[test]
    fn example_four() {
        assert_eq!(
            time_limit(0, Err("Error"), 1000),
            LimitResult::Rejected("Error")
        );
    }
}
