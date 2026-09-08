/// LeetCode #3648 - Minimum Sensors to Cover Grid
fn min_sensors(n: i32, m: i32, k: i32) -> i32 {
    let s = 2 * k + 1;
    let rows = (n + s - 1) / s;
    let cols = (m + s - 1) / s;
    rows * cols
}

fn main() {
    println!("{}", min_sensors(5, 5, 1));
}

#[cfg(test)]
mod tests {
    use super::min_sensors;

    #[test]
    fn example1() {
        assert_eq!(min_sensors(5, 5, 1), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(min_sensors(2, 2, 2), 1);
    }
}
