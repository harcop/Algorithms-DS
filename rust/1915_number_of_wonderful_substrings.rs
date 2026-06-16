/// LeetCode #1915 - Number of Wonderful Substrings
use std::collections::HashMap;

fn wonderful_substrings(word: String) -> i64 {
    let mut cnt: HashMap<i32, i64> = HashMap::new();
    cnt.insert(0, 1);
    let mut ans = 0i64;
    let mut st = 0i32;
    for c in word.bytes() {
        st ^= 1 << (c - b'a');
        ans += cnt.get(&st).copied().unwrap_or(0);
        for i in 0..10 {
            ans += cnt.get(&(st ^ (1 << i))).copied().unwrap_or(0);
        }
        *cnt.entry(st).or_insert(0) += 1;
    }
    ans
}

fn main() {
    println!("{}", wonderful_substrings("aba".into()));
}

#[cfg(test)]
mod tests {
    use super::wonderful_substrings;

    #[test]
    fn example_one() {
        assert_eq!(wonderful_substrings("aba".into()), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(wonderful_substrings("aabb".into()), 9);
    }

    #[test]
    fn example_three() {
        assert_eq!(wonderful_substrings("he".into()), 2);
    }
}
