/// LeetCode #168 - Excel Sheet Column Title
fn convert_to_title(column_number: i32) -> String {
    let mut n = column_number as i64;
    let mut out = Vec::new();
    while n > 0 {
        n -= 1;
        out.push((b'A' + (n % 26) as u8) as char);
        n /= 26;
    }
    out.iter().rev().collect()
}

fn main() {
    println!("{}", convert_to_title(28));
}

#[cfg(test)]
mod tests {
    use super::convert_to_title;

    #[test]
    fn example_one() {
        assert_eq!(convert_to_title(1), "A");
    }

    #[test]
    fn example_two() {
        assert_eq!(convert_to_title(28), "AB");
    }

    #[test]
    fn example_three() {
        assert_eq!(convert_to_title(701), "ZY");
    }
}
