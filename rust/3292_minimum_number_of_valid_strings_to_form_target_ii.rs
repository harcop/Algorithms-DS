/// LeetCode #3292 - Minimum Number of Valid Strings to Form Target II
use std::cmp::max;
use std::collections::HashSet;

struct Hashing {
    p: Vec<i64>,
    h: Vec<i64>,
    modv: i64,
}

impl Hashing {
    fn new(word: &str, base: i64, modv: i64) -> Self {
        let n = word.len();
        let mut p = vec![0; n + 1];
        let mut h = vec![0; n + 1];
        let bytes = word.as_bytes();
        p[0] = 1;
        for i in 1..=n {
            p[i] = p[i - 1] * base % modv;
            h[i] = (h[i - 1] * base + bytes[i - 1] as i64) % modv;
        }
        Self { p, h, modv }
    }

    fn query(&self, l: usize, r: usize) -> i64 {
        let mut res = self.h[r] - self.h[l - 1] * self.p[r - l + 1] % self.modv;
        if res < 0 {
            res += self.modv;
        }
        res % self.modv
    }
}

fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
    let base = 13331;
    let modv = 998_244_353;
    let hashing = Hashing::new(&target, base, modv);
    let m = words.iter().map(|w| w.len()).max().unwrap_or(0);
    let mut s: Vec<HashSet<i64>> = vec![HashSet::new(); m + 1];
    for w in &words {
        let mut h = 0i64;
        for (j, &b) in w.as_bytes().iter().enumerate() {
            h = (h * base + b as i64) % modv;
            s[j + 1].insert(h);
        }
    }
    let n = target.len();
    let f = |i: usize| -> usize {
        let mut l = 0;
        let mut r = std::cmp::min(n - i, m);
        while l < r {
            let mid = (l + r + 1) >> 1;
            let sub = hashing.query(i + 1, i + mid);
            if s[mid].contains(&sub) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        l
    };
    let mut ans = 0;
    let mut last = 0;
    let mut mx = 0;
    for i in 0..n {
        let dist = f(i);
        mx = max(mx, i + dist);
        if i == last {
            if i == mx {
                return -1;
            }
            last = mx;
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        min_valid_strings(
            vec!["abc".into(), "aaaaa".into(), "bcdef".into()],
            "aabcdabc".into()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_valid_strings;

    #[test]
    fn example1() {
        assert_eq!(
            min_valid_strings(
                vec!["abc".into(), "aaaaa".into(), "bcdef".into()],
                "aabcdabc".into()
            ),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_valid_strings(vec!["abababab".into(), "ab".into()], "ababaababa".into()),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(min_valid_strings(vec!["abcdef".into()], "xyz".into()), -1);
    }
}
