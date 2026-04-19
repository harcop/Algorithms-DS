/// LeetCode #6 - Zigzag Conversion
///
/// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows;
/// read line by line and return the result string.

fn convert(s: String, num_rows: i32) -> String {
    let n = num_rows as usize;
    if n <= 1 || s.len() <= n {
        return s;
    }

    let mut rows: Vec<String> = (0..n).map(|_| String::new()).collect();
    let mut cur = 0i32;
    let mut step = 1i32;

    for c in s.chars() {
        rows[cur as usize].push(c);
        if cur == 0 {
            step = 1;
        } else if cur == num_rows - 1 {
            step = -1;
        }
        cur += step;
    }

    rows.concat()
}

fn main() {
    println!("{}", convert("PAYPALISHIRING".to_string(), 3));
}

#[cfg(test)]
mod tests {
    use super::convert;

    #[test]
    fn example_one() {
        assert_eq!(
            convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI");
    }

    #[test]
    fn example_three() {
        assert_eq!(convert("A".to_string(), 1), "A");
    }
}
