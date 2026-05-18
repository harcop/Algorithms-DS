/// LeetCode #1071 - Greatest Common Divisor of Strings
fn gcd_of_strings(str1: String, str2: String) -> String {
    if str1.clone() + &str2 != str2.clone() + &str1 {
        return String::new();
    }
    let g = gcd(str1.len(), str2.len());
    str1[..g].to_string()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    println!("{}", gcd_of_strings("ABCABC".into(), "ABC".into()));
}

#[cfg(test)]
mod tests {
    use super::gcd_of_strings;

    #[test]
    fn example_one() {
        assert_eq!(gcd_of_strings("ABCABC".into(), "ABC".into()), "ABC");
    }

    #[test]
    fn example_two() {
        assert_eq!(gcd_of_strings("ABABAB".into(), "ABAB".into()), "AB");
    }
}
