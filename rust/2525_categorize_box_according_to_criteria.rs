/// LeetCode #2525 - Categorize Box According to Criteria
fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
    let is_bulky = length >= 10_000
        || width >= 10_000
        || height >= 10_000
        || length as i64 * width as i64 * height as i64 >= 1_000_000_000;
    let is_heavy = mass >= 100;
    match (is_bulky, is_heavy) {
        (true, true) => "Both".to_string(),
        (true, false) => "Bulky".to_string(),
        (false, true) => "Heavy".to_string(),
        (false, false) => "Neither".to_string(),
    }
}

fn main() {
    println!("{}", categorize_box(1000, 35, 700, 300));
}

#[cfg(test)]
mod tests {
    use super::categorize_box;

    #[test]
    fn example_one() {
        assert_eq!(categorize_box(1000, 35, 700, 300), "Heavy");
    }

    #[test]
    fn example_two() {
        assert_eq!(categorize_box(200, 50, 200, 50), "Neither");
    }
}
