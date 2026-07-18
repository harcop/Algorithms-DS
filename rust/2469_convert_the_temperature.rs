/// LeetCode #2469 - Convert the Temperature
fn convert_temperature(celsius: f64) -> Vec<f64> {
    vec![celsius + 273.15, celsius * 1.8 + 32.0]
}

fn main() {
    println!("{:?}", convert_temperature(36.50));
}

#[cfg(test)]
mod tests {
    use super::convert_temperature;

    #[test]
    fn example_one() {
        let result = convert_temperature(36.50);
        assert!((result[0] - 309.65).abs() < 1e-5);
        assert!((result[1] - 97.70).abs() < 1e-5);
    }

    #[test]
    fn example_two() {
        let result = convert_temperature(122.11);
        assert!((result[0] - 395.26).abs() < 1e-5);
        assert!((result[1] - 251.798).abs() < 1e-5);
    }
}
