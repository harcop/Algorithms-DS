/// LeetCode #2949 - Count Beautiful Substrings II
use std::collections::HashMap;

fn p_sqrt(mut n: i64) -> i64 {
    let mut res = 1i64;
    let mut i = 2i64;
    while i * i <= n {
        let i2 = i * i;
        while n % i2 == 0 {
            res *= i;
            n /= i2;
        }
        if n % i == 0 {
            res *= i;
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        res *= n;
    }
    res
}

fn beautiful_substrings(s: String, k: i32) -> i64 {
    const AEIOU_MASK: i32 = 1_065_233;
    let l = p_sqrt(k as i64 * 4);
    let n = s.len();
    let bytes = s.as_bytes();
    let mut sum = n as i32;
    let mut ans = 0i64;
    let mut counter: HashMap<(i64, i32), i64> = HashMap::new();
    *counter.entry((l - 1, sum)).or_insert(0) += 1;
    for (i, &ch) in bytes.iter().enumerate() {
        let bit = (AEIOU_MASK >> (ch - b'a')) & 1;
        sum += bit * 2 - 1;
        let key = ((i as i64) % l, sum);
        ans += *counter.get(&key).unwrap_or(&0);
        *counter.entry(key).or_insert(0) += 1;
    }
    ans
}

fn main() {
    println!("{}", beautiful_substrings("baeyh".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::beautiful_substrings;

    #[test]
    fn example_one() {
        assert_eq!(beautiful_substrings("baeyh".into(), 2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(beautiful_substrings("abba".into(), 1), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(beautiful_substrings("bcdf".into(), 1), 0);
    }
}
