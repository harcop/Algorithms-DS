/// LeetCode #3088 - Make String Anti-Palindrome
fn make_anti_palindrome(s: String) -> String {
    let n = s.len();
    let half = n / 2;
    let mut arr: Vec<char> = s.chars().collect();
    arr.sort_unstable();

    let mut freq = [0usize; 26];
    for &c in &arr {
        freq[(c as u8 - b'a') as usize] += 1;
        if freq[(c as u8 - b'a') as usize] > half {
            return "-1".into();
        }
    }

    for i in 0..half {
        if arr[i] == arr[n - 1 - i] {
            let mut j = n - 1 - i;
            while j < n && arr[j] == arr[i] {
                j += 1;
            }
            if j == n {
                return "-1".into();
            }
            arr.swap(n - 1 - i, j);
        }
    }

    for i in 0..half {
        if arr[i] == arr[n - 1 - i] {
            return "-1".into();
        }
    }

    arr.into_iter().collect()
}

fn main() {
    println!("{}", make_anti_palindrome("abca".into()));
}

#[cfg(test)]
mod tests {
    use super::make_anti_palindrome;

    #[test]
    fn example1() {
        assert_eq!(make_anti_palindrome("abca".into()), "aabc");
    }

    #[test]
    fn example2() {
        assert_eq!(make_anti_palindrome("abba".into()), "aabb");
    }

    #[test]
    fn example3() {
        assert_eq!(make_anti_palindrome("cccd".into()), "-1");
    }
}
