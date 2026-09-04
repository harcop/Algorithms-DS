/// LeetCode #564 - Find the Closest Palindrome
use std::collections::HashSet;

fn make_palindrome(prefix: i64, even: bool) -> i64 {
    let mut pal = prefix;
    let mut x = if even { prefix } else { prefix / 10 };
    while x > 0 {
        pal = pal * 10 + x % 10;
        x /= 10;
    }
    pal
}

fn nearest_palindromic(n: String) -> String {
    let len = n.len();
    let num: i64 = n.parse().unwrap();
    let mut cands = HashSet::new();
    cands.insert(10i64.pow(len as u32 - 1) - 1);
    cands.insert(10i64.pow(len as u32) + 1);
    let prefix: i64 = n[..(len + 1) / 2].parse().unwrap();
    for p in [prefix - 1, prefix, prefix + 1] {
        if p >= 0 {
            cands.insert(make_palindrome(p, len % 2 == 0));
        }
    }
    cands.remove(&num);
    let mut best = i64::MAX;
    let mut best_diff = i64::MAX;
    for c in cands {
        let d = (c - num).abs();
        if d < best_diff || (d == best_diff && c < best) {
            best_diff = d;
            best = c;
        }
    }
    best.to_string()
}

fn main() {
    println!("{}", nearest_palindromic("123".into()));
}

#[cfg(test)]
mod tests {
    use super::nearest_palindromic;

    #[test]
    fn example_one() {
        assert_eq!(nearest_palindromic("123".into()), "121");
    }

    #[test]
    fn example_two() {
        assert_eq!(nearest_palindromic("1".into()), "0");
    }
}
