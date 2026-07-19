/// LeetCode #2496 - Maximum Value of a String in an Array
fn maximum_value(strs: Vec<String>) -> i32 {
    let mut ans = 0;
    for s in strs {
        let value = if s.bytes().all(|c| c.is_ascii_digit()) {
            s.parse::<i32>().unwrap_or(0)
        } else {
            s.len() as i32
        };
        ans = ans.max(value);
    }
    ans
}

fn main() {
    println!(
        "{}",
        maximum_value(vec![
            "alic3".to_string(),
            "bob".to_string(),
            "3".to_string(),
            "4".to_string(),
            "00000".to_string()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_value;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_value(vec![
                "alic3".to_string(),
                "bob".to_string(),
                "3".to_string(),
                "4".to_string(),
                "00000".to_string()
            ]),
            5
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_value(vec![
                "1".to_string(),
                "01".to_string(),
                "001".to_string(),
                "0001".to_string()
            ]),
            1
        );
    }
}
