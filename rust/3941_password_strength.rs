/// LeetCode #3941 - Password Strength
fn password_strength(password: String) -> i32 {
    let mut seen = [false; 128];
    let mut ans = 0i32;
    for &b in password.as_bytes() {
        if seen[b as usize] {
            continue;
        }
        seen[b as usize] = true;
        ans += if b.is_ascii_lowercase() {
            1
        } else if b.is_ascii_uppercase() {
            2
        } else if b.is_ascii_digit() {
            3
        } else {
            5
        };
    }
    ans
}

fn main() {
    println!("{}", password_strength("aA1!".into()));
}

#[cfg(test)]
mod tests {
    use super::password_strength;

    #[test]
    fn example1() {
        assert_eq!(password_strength("aA1!".into()), 11);
    }

    #[test]
    fn example2() {
        assert_eq!(password_strength("bbB11#".into()), 11);
    }
}
