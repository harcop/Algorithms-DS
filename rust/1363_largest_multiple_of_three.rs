/// LeetCode #1363 - Largest Multiple Of Three

fn largest_multiple_of_three(digits: Vec<i32>) -> String {
    let mut digits = digits;
    digits.sort_unstable_by(|a, b| b.cmp(a));
    let sum: i32 = digits.iter().sum();
    let rem = sum % 3;
    if rem == 0 {
        return build_string(&digits, &vec![false; digits.len()]);
    }
    let mut best = String::new();
    for i in (0..digits.len()).rev() {
        if digits[i] % 3 == rem {
            let mut drop = vec![false; digits.len()];
            drop[i] = true;
            let candidate = build_string(&digits, &drop);
            if better_multiple(&candidate, &best) {
                best = candidate;
            }
        }
    }
    for i in (0..digits.len()).rev() {
        for j in (0..digits.len()).rev() {
            if i == j {
                continue;
            }
            if (digits[i] + digits[j]) % 3 == rem {
                let mut drop = vec![false; digits.len()];
                drop[i] = true;
                drop[j] = true;
                let candidate = build_string(&digits, &drop);
                if better_multiple(&candidate, &best) {
                    best = candidate;
                }
            }
        }
    }
    best
}

fn better_multiple(a: &str, b: &str) -> bool {
    if a.is_empty() {
        return false;
    }
    if b.is_empty() {
        return true;
    }
    if a.len() != b.len() {
        return a.len() > b.len();
    }
    a > b
}

fn build_string(digits: &[i32], drop: &[bool]) -> String {
    let out: String = digits
        .iter()
        .enumerate()
        .filter(|(i, _)| !drop[*i])
        .map(|(_, &d)| char::from(b'0' + d as u8))
        .collect();
    if out.is_empty() {
        return String::new();
    }
    if out.as_bytes()[0] == b'0' {
        if out.bytes().all(|b| b == b'0') {
            return "0".into();
        }
        return String::new();
    }
    out
}

fn main() {
    println!("{}", largest_multiple_of_three(vec![8, 1, 9]));
}

#[cfg(test)]
mod tests {
    use super::largest_multiple_of_three;

    #[test]
    fn example_one() {
        assert_eq!(largest_multiple_of_three(vec![8, 1, 9]), "981");
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_multiple_of_three(vec![8, 6, 7, 1, 0]), "8760");
    }
}
