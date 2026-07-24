/// LeetCode #2650 - Design Cancellable Function (JS problem; simulated cancel analogue)
#[derive(Debug, PartialEq, Eq)]
enum Step {
    /// Wait `ms`, then produce Ok(value) or Err(message).
    Yield { ms: i32, result: Result<i32, &'static str> },
    /// Return a final value (success path).
    Return(i32),
    /// Throw an error (using previous yielded value as format arg if needed).
    Throw(&'static str),
}

#[derive(Debug, PartialEq, Eq)]
enum RunResult {
    Resolved(i32),
    Rejected(&'static str),
}

/// Simulate a cancellable generator.
/// `steps` is the sequence of yields/returns; `cancelled_at` is absolute ms or None.
/// Catching cancel: if `catch_cancel` is true, returning the accumulator so far on cancel.
fn cancellable(
    steps: &[Step],
    cancelled_at: Option<i32>,
    catch_cancel: bool,
) -> RunResult {
    let mut t = 0i32;
    let mut last_val = 0i32;
    let mut acc = 0i32;
    let mut i = 0usize;

    while i < steps.len() {
        match &steps[i] {
            Step::Yield { ms, result } => {
                let finish = t + ms;
                if let Some(c) = cancelled_at {
                    if c < finish && c >= t {
                        if catch_cancel {
                            return RunResult::Resolved(acc);
                        }
                        return RunResult::Rejected("Cancelled");
                    }
                }
                t = finish;
                match result {
                    Ok(v) => {
                        last_val = *v;
                        acc += *v;
                    }
                    Err(e) => {
                        // propagate into generator; if next steps catch, continue
                        // For simplicity: if remaining steps exist after this Err yield,
                        // treat as caught and continue from next step.
                        if i + 1 < steps.len() {
                            i += 1;
                            continue;
                        }
                        return RunResult::Rejected(e);
                    }
                }
                i += 1;
            }
            Step::Return(v) => return RunResult::Resolved(*v),
            Step::Throw(msg) => {
                let _ = last_val;
                return RunResult::Rejected(msg);
            }
        }
    }
    RunResult::Resolved(acc)
}

fn main() {
    println!("{:?}", cancellable(&[Step::Return(42)], Some(100), false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            cancellable(&[Step::Return(42)], Some(100), false),
            RunResult::Resolved(42)
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            cancellable(
                &[
                    Step::Yield {
                        ms: 0,
                        result: Ok(0)
                    }, // "Hello" placeholder timing
                    Step::Throw("Error: Hello"),
                ],
                None,
                false
            ),
            RunResult::Rejected("Error: Hello")
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            cancellable(
                &[
                    Step::Yield {
                        ms: 200,
                        result: Ok(0)
                    },
                    Step::Return(0), // "Success" unused
                ],
                Some(100),
                false
            ),
            RunResult::Rejected("Cancelled")
        );
    }

    #[test]
    fn example_four() {
        assert_eq!(
            cancellable(
                &[
                    Step::Yield {
                        ms: 100,
                        result: Ok(0)
                    },
                    Step::Yield {
                        ms: 0,
                        result: Ok(1)
                    },
                    Step::Yield {
                        ms: 100,
                        result: Ok(0)
                    },
                    Step::Yield {
                        ms: 0,
                        result: Ok(1)
                    },
                    Step::Return(2),
                ],
                None,
                false
            ),
            RunResult::Resolved(2)
        );
    }

    #[test]
    fn example_five() {
        assert_eq!(
            cancellable(
                &[
                    Step::Yield {
                        ms: 100,
                        result: Ok(0)
                    },
                    Step::Yield {
                        ms: 0,
                        result: Ok(1)
                    },
                    Step::Yield {
                        ms: 100,
                        result: Ok(0)
                    },
                    Step::Yield {
                        ms: 0,
                        result: Ok(1)
                    },
                ],
                Some(150),
                true
            ),
            RunResult::Resolved(1)
        );
    }

    #[test]
    fn example_six() {
        assert_eq!(
            cancellable(
                &[
                    Step::Yield {
                        ms: 0,
                        result: Err("Promise Rejected")
                    },
                    Step::Yield {
                        ms: 0,
                        result: Ok(2)
                    },
                    Step::Yield {
                        ms: 0,
                        result: Ok(2)
                    },
                    Step::Return(4),
                ],
                None,
                false
            ),
            RunResult::Resolved(4)
        );
    }
}
