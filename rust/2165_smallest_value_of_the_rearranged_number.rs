/// LeetCode #2165 - Smallest Value of the Rearranged Number
fn smallest_number(num: i64) -> i64 {
    if num == 0 {
        return 0;
    }

    let negative = num < 0;
    let mut digits: Vec<i64> = num
        .abs()
        .to_string()
        .bytes()
        .map(|b| (b - b'0') as i64)
        .collect();
    if negative {
        digits.sort_unstable_by(|a, b| b.cmp(a));
        return -digits.into_iter().fold(0, |acc, d| acc * 10 + d);
    }

    digits.sort_unstable();
    let first_non_zero = digits.iter().position(|&d| d != 0).unwrap();
    digits.swap(0, first_non_zero);
    digits.into_iter().fold(0, |acc, d| acc * 10 + d)
}

fn main() {
    println!("{}", smallest_number(310));
}

#[cfg(test)]
mod tests {
    use super::smallest_number;

    #[test]
    fn example_one() {
        assert_eq!(smallest_number(310), 103);
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_number(-7605), -7650);
    }
}
