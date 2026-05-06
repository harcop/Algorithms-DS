/// LeetCode #405 - Convert a Number to Hexadecimal
fn to_hex(num: i32) -> String {
    if num == 0 {
        return "0".into();
    }
    let mut n = num as u32;
    let mut out = String::new();
    let hex = b"0123456789abcdef";
    while n != 0 {
        out.push(hex[(n & 0xF) as usize] as char);
        n >>= 4;
    }
    out.chars().rev().collect()
}

fn main() {
    println!("{}", to_hex(26));
}

#[cfg(test)]
mod tests {
    use super::to_hex;

    #[test]
    fn example_one() {
        assert_eq!(to_hex(26), "1a");
    }

    #[test]
    fn example_two() {
        assert_eq!(to_hex(-1), "ffffffff");
    }
}
