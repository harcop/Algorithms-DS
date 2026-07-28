/// LeetCode #2725 - Interval Cancellation (JS problem; simulated timer analogue)
/// Call `run` at t=0, then every `interval` ms until `cancel_at`.
fn cancellable_interval(run: impl Fn() -> i32, interval: i32, cancel_at: i32) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let mut t = 0;
    while t < cancel_at {
        result.push((t, run()));
        t += interval;
    }
    result
}

fn main() {
    println!("{:?}", cancellable_interval(|| 4 * 2, 35, 190));
}

#[cfg(test)]
mod tests {
    use super::cancellable_interval;

    #[test]
    fn example_one() {
        assert_eq!(
            cancellable_interval(|| 4 * 2, 35, 190),
            vec![
                (0, 8),
                (35, 8),
                (70, 8),
                (105, 8),
                (140, 8),
                (175, 8),
            ]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            cancellable_interval(|| 2 * 5, 30, 165),
            vec![
                (0, 10),
                (30, 10),
                (60, 10),
                (90, 10),
                (120, 10),
                (150, 10),
            ]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            cancellable_interval(|| 5 + 1 + 3, 50, 180),
            vec![(0, 9), (50, 9), (100, 9), (150, 9)]
        );
    }
}
