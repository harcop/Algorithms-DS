/// LeetCode #3503 - Longest Palindrome After Substring Concatenation I
fn expand(s: &[u8], g: &mut [i32], mut l: i32, mut r: i32) {
    let n = s.len() as i32;
    while l >= 0 && r < n && s[l as usize] == s[r as usize] {
        g[l as usize] = g[l as usize].max(r - l + 1);
        l -= 1;
        r += 1;
    }
}

fn calc(s: &[u8]) -> Vec<i32> {
    let n = s.len();
    let mut g = vec![0; n];
    for i in 0..n {
        expand(s, &mut g, i as i32, i as i32);
        expand(s, &mut g, i as i32, i as i32 + 1);
    }
    g
}

fn longest_palindrome(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t: Vec<u8> = t.bytes().rev().collect();
    let m = s.len();
    let n = t.len();
    let g1 = calc(s);
    let g2 = calc(&t);
    let mut ans = *g1.iter().chain(g2.iter()).max().unwrap();
    let mut f = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if s[i - 1] == t[j - 1] {
                f[i][j] = f[i - 1][j - 1] + 1;
                ans = ans.max(f[i][j] * 2 + if i >= m { 0 } else { g1[i] });
                ans = ans.max(f[i][j] * 2 + if j >= n { 0 } else { g2[j] });
            }
        }
    }
    ans
}

fn main() {
    println!("{}", longest_palindrome("a".into(), "a".into()));
}

#[cfg(test)]
mod tests {
    use super::longest_palindrome;

    #[test]
    fn example1() {
        assert_eq!(longest_palindrome("a".into(), "a".into()), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_palindrome("abc".into(), "def".into()), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(longest_palindrome("b".into(), "aaaa".into()), 4);
    }

    #[test]
    fn example4() {
        assert_eq!(longest_palindrome("abcde".into(), "ecdba".into()), 5);
    }
}
