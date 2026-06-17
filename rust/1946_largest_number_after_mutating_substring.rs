/// LeetCode #1946 - Largest Number After Mutating Substring
fn maximum_number(num: String, change: Vec<i32>) -> String {
    let bytes = num.as_bytes();
    let mut ans: Vec<u8> = bytes.to_vec();
    let mut changed = false;
    for i in 0..bytes.len() {
        let d = change[(bytes[i] - b'0') as usize] as u8 + b'0';
        if changed && d < bytes[i] {
            break;
        }
        if d > bytes[i] {
            changed = true;
            ans[i] = d;
        }
    }
    String::from_utf8(ans).unwrap()
}

fn main() {
    println!(
        "{}",
        maximum_number("132".into(), vec![9, 8, 5, 0, 3, 6, 4, 2, 6, 8])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_number;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_number("132".into(), vec![9, 8, 5, 0, 3, 6, 4, 2, 6, 8]),
            "832"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_number("021".into(), vec![9, 4, 3, 5, 7, 2, 1, 9, 0, 6]),
            "934"
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            maximum_number("5".into(), vec![1, 4, 7, 5, 3, 2, 5, 6, 9, 4]),
            "5"
        );
    }
}
