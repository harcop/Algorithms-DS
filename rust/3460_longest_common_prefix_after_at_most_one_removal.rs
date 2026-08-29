/// LeetCode #3460 - Longest Common Prefix After at Most One Removal
fn longest_common_prefix(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut i = 0;
    let mut j = 0;
    let mut rem = false;
    while i < s.len() && j < t.len() {
        if s[i] != t[j] {
            if rem {
                break;
            }
            rem = true;
        } else {
            j += 1;
        }
        i += 1;
    }
    j as i32
}

fn main() {
    println!(
        "{}",
        longest_common_prefix("madxa".into(), "madam".into())
    );
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;

    #[test]
    fn example1() {
        assert_eq!(longest_common_prefix("madxa".into(), "madam".into()), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(
            longest_common_prefix("leetcode".into(), "eetcode".into()),
            7
        );
    }

    #[test]
    fn example3() {
        assert_eq!(longest_common_prefix("one".into(), "one".into()), 3);
    }

    #[test]
    fn example4() {
        assert_eq!(longest_common_prefix("a".into(), "b".into()), 0);
    }
}
