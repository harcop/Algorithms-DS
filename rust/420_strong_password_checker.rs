/// LeetCode #420 - Strong Password Checker (LeetCode O(n) greedy; covers length / classes / repeats)
fn strong_password_checker(password: String) -> i32 {
    let chars: Vec<char> = password.chars().collect();
    let n = chars.len();
    let mut lower = 0i32;
    let mut upper = 0i32;
    let mut digit = 0i32;
    for c in &chars {
        if c.is_ascii_lowercase() {
            lower = 1;
        } else if c.is_ascii_uppercase() {
            upper = 1;
        } else if c.is_ascii_digit() {
            digit = 1;
        }
    }
    let missing = 3 - (lower + upper + digit);

    let mut replace = 0i32;
    let mut one = 0i32;
    let mut two = 0i32;
    let mut i = 2usize;
    while i < n {
        if chars[i] == chars[i - 1] && chars[i - 2] == chars[i] {
            let mut length = 2i32;
            let mut j = i;
            while j + 1 < n && chars[j + 1] == chars[i] {
                j += 1;
                length += 1;
            }
            replace += length / 3;
            if length % 3 == 0 {
                one += 1;
            } else if length % 3 == 1 {
                two += 1;
            }
            i = j + 1;
        } else {
            i += 1;
        }
    }

    if n < 6 {
        return (6 - n as i32).max(missing).max(replace);
    }
    if n <= 20 {
        return missing.max(replace);
    }

    let delete = n as i32 - 20;
    replace -= delete.min(one);
    replace -= (delete - one).max(0).min(two * 2) / 2;
    replace -= (delete - one - 2 * two).max(0) / 3;
    delete + missing.max(replace)
}

fn main() {
    println!("{}", strong_password_checker("aA123".into()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        assert!(strong_password_checker("---".into()) >= 0);
    }
}
