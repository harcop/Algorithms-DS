/// LeetCode #3894 - Traffic Signal Color

fn traffic_signal_color(timer: i32) -> String {
    match timer {
        0 => "Green".into(),
        30 => "Orange".into(),
        t if t > 30 && t <= 90 => "Red".into(),
        _ => "Invalid".into(),
    }
}

fn main() {
    println!("{}", traffic_signal_color(60));
}

#[cfg(test)]
mod tests {
    use super::traffic_signal_color;

    #[test]
    fn example_red() {
        assert_eq!(traffic_signal_color(60), "Red");
    }

    #[test]
    fn example_invalid() {
        assert_eq!(traffic_signal_color(5), "Invalid");
    }

    #[test]
    fn green() {
        assert_eq!(traffic_signal_color(0), "Green");
    }

    #[test]
    fn orange() {
        assert_eq!(traffic_signal_color(30), "Orange");
    }
}
