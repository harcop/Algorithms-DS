/// LeetCode #2715 - Timeout Cancellation (JS problem; simulated timer analogue)
/// Simulate: schedule fn at time `t`; cancel at `cancel_at`. Return results if executed.
fn cancellable_simulate(
    run: impl FnOnce() -> i32,
    t: i32,
    cancel_at: i32,
) -> Vec<(i32, i32)> {
    if cancel_at < t {
        vec![]
    } else {
        vec![(t, run())]
    }
}

fn main() {
    println!("{:?}", cancellable_simulate(|| 2 * 5, 20, 50));
}

#[cfg(test)]
mod tests {
    use super::cancellable_simulate;

    #[test]
    fn example_one() {
        assert_eq!(cancellable_simulate(|| 2 * 5, 20, 50), vec![(20, 10)]);
    }

    #[test]
    fn example_two() {
        assert_eq!(cancellable_simulate(|| 2 * 2, 100, 50), vec![]);
    }

    #[test]
    fn example_three() {
        assert_eq!(cancellable_simulate(|| 2 * 4, 30, 100), vec![(30, 8)]);
    }
}
