/// LeetCode #1058 - Minimize Rounding Error to Meet Target
fn minimize_rounding_error(prices: Vec<String>, target: i32) -> String {
    let mut floor_sum = 0i32;
    let mut diffs: Vec<f64> = Vec::new();
    for p in prices {
        let v: f64 = p.parse().unwrap();
        let fl = v.floor();
        floor_sum += fl as i32;
        diffs.push(v - fl);
    }
    let need = target - floor_sum;
    if need < 0 || need as usize > diffs.len() {
        return "-1".into();
    }
    diffs.sort_by(|a, b| (1.0 - a).partial_cmp(&(1.0 - b)).unwrap());
    let mut err = 0.0f64;
    for (i, &d) in diffs.iter().enumerate() {
        if i < need as usize {
            err += 1.0 - d;
        } else {
            err += d;
        }
    }
    format!("{:.3}", err)
}

fn main() {
    println!("{}", minimize_rounding_error(vec!["0.70".into(), "2.80".into(), "4.90".into()], 6));
}

#[cfg(test)]
mod tests {
    use super::minimize_rounding_error;

    #[test]
    fn example_one() {
        assert_eq!(
            minimize_rounding_error(vec!["0.70".into(), "2.80".into(), "4.90".into()], 6),
            "2.400"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimize_rounding_error(vec!["0.25".into(), "0.25".into(), "0.25".into(), "0.25".into()], 1),
            "1.500"
        );
    }
}
