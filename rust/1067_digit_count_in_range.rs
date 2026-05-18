/// LeetCode #1067 - Digit Count in Range
fn digits_count(d: i32, low: i32, high: i32) -> i32 {
    let mut count = 0i32;
    for x in low..=high {
        count += x
            .to_string()
            .bytes()
            .filter(|&b| (b - b'0') as i32 == d)
            .count() as i32;
    }
    count
}

fn main() {
    println!("{}", digits_count(1, 1, 13));
}

#[cfg(test)]
mod tests {
    use super::digits_count;

    #[test]
    fn example_one() {
        assert_eq!(digits_count(1, 1, 13), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(digits_count(3, 100, 250), 35);
    }
}
