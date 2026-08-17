/// LeetCode #3233 - Find the Count of Numbers Which Are Not Special
fn non_special_count(l: i32, r: i32) -> i32 {
    let m = 31623usize;
    let mut primes = vec![true; m + 1];
    primes[0] = false;
    primes[1] = false;
    for i in 2..=m {
        if primes[i] {
            let mut j = i * 2;
            while j <= m {
                primes[j] = false;
                j += i;
            }
        }
    }
    let isqrt = |x: i64| -> i64 { (x as f64).sqrt() as i64 };
    let mut lo = isqrt(l as i64);
    if lo * lo < l as i64 {
        lo += 1;
    }
    let mut hi = isqrt(r as i64);
    if hi * hi > r as i64 {
        hi -= 1;
    }
    let mut cnt = 0;
    if lo <= hi {
        for i in lo..=hi {
            if primes[i as usize] {
                cnt += 1;
            }
        }
    }
    r - l + 1 - cnt
}

fn main() {
    println!("{}", non_special_count(5, 7));
}

#[cfg(test)]
mod tests {
    use super::non_special_count;

    #[test]
    fn example1() {
        assert_eq!(non_special_count(5, 7), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(non_special_count(4, 16), 11);
    }
}
