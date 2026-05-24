/// LeetCode #1271 - Hexspeak
fn to_hexspeak(num: String) -> String {
    let n: u128 = num.parse().expect("valid decimal");
    let hex = format!("{:X}", n);
    if hex.chars().any(|c| matches!(c, '2'..='9')) {
        return "ERROR".into();
    }
    hex.replace('0', "O").replace('1', "I")
}

fn main() {
    println!("{}", to_hexspeak("257".into()));
}

#[cfg(test)]
mod tests {
    use super::to_hexspeak;

    #[test]
    fn example_one() {
        assert_eq!(to_hexspeak("257".into()), "IOI");
    }

    #[test]
    fn example_two() {
        assert_eq!(to_hexspeak("3".into()), "ERROR");
    }
}
