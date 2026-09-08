/// LeetCode #3646 - Next Special Palindrome Number
fn next_permutation(a: &mut [i32]) -> bool {
    let n = a.len();
    if n < 2 {
        return false;
    }
    let mut i = n - 2;
    loop {
        if a[i] < a[i + 1] {
            break;
        }
        if i == 0 {
            a.reverse();
            return false;
        }
        i -= 1;
    }
    let mut j = n - 1;
    while a[j] <= a[i] {
        j -= 1;
    }
    a.swap(i, j);
    a[i + 1..].reverse();
    true
}

fn special_palindromes() -> Vec<i64> {
    let mut result = Vec::new();
    for mask in 1..(1 << 9) {
        let mut half = Vec::new();
        let mut mid: Option<i32> = None;
        let mut odd = 0;
        let mut total = 0;
        for i in 0..9 {
            if mask & (1 << i) == 0 {
                continue;
            }
            let d = (i + 1) as i32;
            total += d;
            if d % 2 == 1 {
                odd += 1;
                mid = Some(d);
            }
            for _ in 0..(d / 2) {
                half.push(d);
            }
        }
        if odd > 1 || total > 18 {
            continue;
        }
        half.sort_unstable();
        loop {
            let mut s = String::new();
            for &d in &half {
                s.push(char::from_digit(d as u32, 10).unwrap());
            }
            let mid_s = mid.map(|d| d.to_string()).unwrap_or_default();
            let rev: String = s.chars().rev().collect();
            let full = format!("{}{}{}", s, mid_s, rev);
            if !full.is_empty() {
                if let Ok(v) = full.parse::<i64>() {
                    result.push(v);
                }
            }
            if !next_permutation(&mut half) {
                break;
            }
        }
    }
    result.sort_unstable();
    result.dedup();
    result
}

fn special_palindrome(n: i64) -> i64 {
    let pals = special_palindromes();
    let i = pals.partition_point(|&x| x <= n);
    pals[i]
}

fn main() {
    println!("{}", special_palindrome(2));
}

#[cfg(test)]
mod tests {
    use super::special_palindrome;

    #[test]
    fn example1() {
        assert_eq!(special_palindrome(2), 22);
    }

    #[test]
    fn example2() {
        assert_eq!(special_palindrome(33), 212);
    }

    #[test]
    fn zero() {
        assert_eq!(special_palindrome(0), 1);
    }
}
