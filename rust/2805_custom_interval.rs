/// LeetCode #2805 - Custom Interval (JS problem; Rust analogue)
/// Returns the call times of `fn` before `cancel_time`, where the k-th interval
/// waits `delay + period * k` ms after the previous call (first at `delay`).
fn custom_interval_times(delay: i32, period: i32, cancel_time: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut t = 0;
    let mut count = 0;
    loop {
        let wait = delay + period * count;
        t += wait;
        if t >= cancel_time {
            break;
        }
        result.push(t);
        count += 1;
    }
    result
}

fn main() {
    println!("{:?}", custom_interval_times(50, 20, 225));
}

#[cfg(test)]
mod tests {
    use super::custom_interval_times;

    #[test]
    fn example_one() {
        assert_eq!(custom_interval_times(50, 20, 225), vec![50, 120, 210]);
    }

    #[test]
    fn example_two() {
        assert_eq!(custom_interval_times(20, 20, 150), vec![20, 60, 120]);
    }

    #[test]
    fn example_three() {
        assert_eq!(custom_interval_times(100, 200, 500), vec![100, 400]);
    }
}
