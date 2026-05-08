/// LeetCode #504 - Base 7
fn convert_to_base7(num: i32) -> String {
    if num == 0 {
        return "0".into();
    }
    let mut n = num.abs();
    let mut out = String::new();
    while n > 0 {
        out.push(char::from(b'0' + (n % 7) as u8));
        n /= 7;
    }
    if num < 0 {
        out.push('-');
    }
    out.chars().rev().collect()
}

fn main() {
    println!("{}", convert_to_base7(100));
}

#[cfg(test)]
mod tests {
    use super::convert_to_base7;

    #[test]
    fn example_one() {
        assert_eq!(convert_to_base7(100), "202");
    }

    #[test]
    fn example_two() {
        assert_eq!(convert_to_base7(-7), "-10");
    }
}
