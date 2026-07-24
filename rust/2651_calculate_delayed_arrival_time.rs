/// LeetCode #2651 - Calculate Delayed Arrival Time
fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
    (arrival_time + delayed_time) % 24
}

fn main() {
    println!("{}", find_delayed_arrival_time(15, 5));
}

#[cfg(test)]
mod tests {
    use super::find_delayed_arrival_time;

    #[test]
    fn example_one() {
        assert_eq!(find_delayed_arrival_time(15, 5), 20);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_delayed_arrival_time(13, 11), 0);
    }
}
