/// LeetCode #1256 - Encode Number
fn encode(num: i32) -> String {
    let bits = format!("{:b}", num + 1);
    bits[1..].to_string()
}

fn main() {
    println!("{}", encode(23));
}

#[cfg(test)]
mod tests {
    use super::encode;

    #[test]
    fn example_one() {
        assert_eq!(encode(23), "1000");
    }

    #[test]
    fn example_two() {
        assert_eq!(encode(107), "101100");
    }
}
