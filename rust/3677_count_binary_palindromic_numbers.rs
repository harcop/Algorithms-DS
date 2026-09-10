/// LeetCode #3677 - Count Binary Palindromic Numbers
fn make_palindrome(half: i64, len: i32) -> i64 {
    let half_bits = (len + 1) / 2;
    let mut x = half;
    let mut t = if len % 2 == 0 { half } else { half >> 1 };
    for _ in 0..(len / 2) {
        x = (x << 1) | (t & 1);
        t >>= 1;
    }
    let _ = half_bits;
    x
}

fn count_binary_palindromes(n: i64) -> i32 {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 1;
    }
    let mut count = 1i32; // 0
    let max_len = 64 - n.leading_zeros() as i32;
    for len in 1..=max_len {
        let half_len = (len + 1) / 2;
        let start = 1i64 << (half_len - 1);
        let end = (1i64 << half_len) - 1;
        for half in start..=end {
            let pal = make_palindrome(half, len);
            if pal > n {
                break;
            }
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", count_binary_palindromes(9));
}

#[cfg(test)]
mod tests {
    use super::count_binary_palindromes;

    #[test]
    fn example1() {
        assert_eq!(count_binary_palindromes(9), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(count_binary_palindromes(0), 1);
    }
}
