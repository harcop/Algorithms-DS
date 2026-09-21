/// LeetCode #3844 - Longest Almost-Palindromic Substring
fn almost_palindromic(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let f = |mut l: i32, mut r: i32| -> i32 {
        while l >= 0 && r < n as i32 && s[l as usize] == s[r as usize] {
            l -= 1;
            r += 1;
        }
        let (mut l1, mut r1) = (l - 1, r);
        let (mut l2, mut r2) = (l, r + 1);
        while l1 >= 0 && r1 < n as i32 && s[l1 as usize] == s[r1 as usize] {
            l1 -= 1;
            r1 += 1;
        }
        while l2 >= 0 && r2 < n as i32 && s[l2 as usize] == s[r2 as usize] {
            l2 -= 1;
            r2 += 1;
        }
        (n as i32).min((r1 - l1 - 1).max(r2 - l2 - 1))
    };
    let mut ans = 0;
    for i in 0..n as i32 {
        ans = ans.max(f(i, i)).max(f(i, i + 1));
    }
    ans
}

fn main() {
    println!("{}", almost_palindromic("abca".into()));
}

#[cfg(test)]
mod tests {
    use super::almost_palindromic;

    #[test]
    fn example1() {
        assert_eq!(almost_palindromic("abca".into()), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(almost_palindromic("abba".into()), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(almost_palindromic("zzabba".into()), 5);
    }
}
