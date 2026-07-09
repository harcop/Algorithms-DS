/// LeetCode #2315 - Count Asterisks
fn count_asterisks(s: String) -> i32 {
    let mut ans = 0;
    let mut ok = 1;
    for &c in s.as_bytes() {
        if c == b'*' {
            ans += ok;
        } else if c == b'|' {
            ok ^= 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_asterisks("l|*e*et|c**o|*de|".to_string()));
}

#[cfg(test)]
mod tests {
    use super::count_asterisks;

    #[test]
    fn example_one() {
        assert_eq!(count_asterisks("l|*e*et|c**o|*de|".to_string()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_asterisks("iamprogrammer".to_string()), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_asterisks("yo|uar|e**|b|e***aut|iful|".to_string()), 5);
    }
}
