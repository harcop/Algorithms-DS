/// LeetCode #1221 - Split a String in Balanced Strings
fn balanced_string_split(s: String) -> i32 {
    let mut bal = 0i32;
    let mut ans = 0i32;
    for c in s.bytes() {
        if c == b'L' {
            bal -= 1;
        } else {
            bal += 1;
        }
        if bal == 0 {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", balanced_string_split("RLRRLLRLRL".into()));
}

#[cfg(test)]
mod tests {
    use super::balanced_string_split;

    #[test]
    fn example_one() {
        assert_eq!(balanced_string_split("RLRRLLRLRL".into()), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(balanced_string_split("RLRLRLRLRL".into()), 5);
    }
}
