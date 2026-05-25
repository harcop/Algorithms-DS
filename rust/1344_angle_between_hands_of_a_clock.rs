/// LeetCode #1344 - Angle Between Hands Of A Clock

fn angle_clock(hour: i32, minutes: i32) -> f64 {
    let h = (hour % 12) as f64;
    let minute_angle = minutes as f64 * 6.0;
    let hour_angle = h * 30.0 + minutes as f64 * 0.5;
    let diff = (hour_angle - minute_angle).abs();
    diff.min(360.0 - diff)
}

fn main() {
    println!("{}", angle_clock(12, 30));
}

#[cfg(test)]
mod tests {
    use super::angle_clock;

    #[test]
    fn example_one() {
        assert!((angle_clock(12, 30) - 165.0).abs() < 1e-5);
    }

    #[test]
    fn example_two() {
        assert!((angle_clock(3, 30) - 75.0).abs() < 1e-5);
    }
}
