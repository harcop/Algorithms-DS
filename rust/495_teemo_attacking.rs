/// LeetCode #495 - Teemo Attacking
fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    if time_series.is_empty() {
        return 0;
    }
    let mut ans = 0i32;
    for i in 1..time_series.len() {
        ans += (time_series[i] - time_series[i - 1]).min(duration);
    }
    ans + duration
}

fn main() {
    println!("{}", find_poisoned_duration(vec![1, 4], 2));
}

#[cfg(test)]
mod tests {
    use super::find_poisoned_duration;

    #[test]
    fn example_one() {
        assert_eq!(find_poisoned_duration(vec![1, 4], 2), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_poisoned_duration(vec![1, 2], 2), 3);
    }
}
