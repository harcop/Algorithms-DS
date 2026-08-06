/// LeetCode #3035 - Maximum Palindromes After Operations
fn max_palindromes(words: Vec<String>) -> i32 {
    let mut s: i32 = words.iter().map(|w| w.len() as i32).sum();
    let mut mask = 0u32;
    for w in &words {
        for c in w.bytes() {
            mask ^= 1 << (c - b'a') as u32;
        }
    }
    s -= mask.count_ones() as i32;

    let mut lengths: Vec<i32> = words.iter().map(|w| w.len() as i32).collect();
    lengths.sort();

    let mut ans = 0;
    for len in lengths {
        s -= (len / 2) * 2;
        if s < 0 {
            break;
        }
        ans += 1;
    }
    ans
}

fn main() {
    let words = vec!["abbb".into(), "ba".into(), "aa".into()];
    println!("{}", max_palindromes(words));
}

#[cfg(test)]
mod tests {
    use super::max_palindromes;

    #[test]
    fn example1() {
        assert_eq!(
            max_palindromes(vec!["abbb".into(), "ba".into(), "aa".into()]),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(max_palindromes(vec!["abc".into(), "ab".into()]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(max_palindromes(vec!["cd".into(), "ef".into(), "a".into()]), 1);
    }
}
