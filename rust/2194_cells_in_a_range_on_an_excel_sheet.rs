/// LeetCode #2194 - Cells in a Range on an Excel Sheet
fn cells_in_range(s: String) -> Vec<String> {
    let bytes = s.as_bytes();
    let c1 = bytes[0];
    let r1 = bytes[1] - b'0';
    let c2 = bytes[3];
    let r2 = bytes[4] - b'0';

    let mut ans = Vec::new();
    for col in c1..=c2 {
        for row in r1..=r2 {
            ans.push(format!("{}{}", col as char, row));
        }
    }
    ans
}

fn main() {
    println!("{:?}", cells_in_range("K1:L2".into()));
}

#[cfg(test)]
mod tests {
    use super::cells_in_range;

    #[test]
    fn example_one() {
        assert_eq!(
            cells_in_range("K1:L2".into()),
            vec![
                "K1".to_string(),
                "K2".to_string(),
                "L1".to_string(),
                "L2".to_string()
            ]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            cells_in_range("A1:F1".into()),
            vec![
                "A1".to_string(),
                "B1".to_string(),
                "C1".to_string(),
                "D1".to_string(),
                "E1".to_string(),
                "F1".to_string()
            ]
        );
    }
}
