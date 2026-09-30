/// LeetCode #4022 - K-th Digit in Infinite String
fn kth_digit(mut k: i64) -> i32 {
    if k <= 9 {
        return k as i32;
    }
    k -= 9;
    let mut d = 2i64;
    let mut start = 1i64;
    loop {
        let cnt = 9 * pow10(d - 2);
        let size = 10 * d;
        if k <= cnt * size {
            let b = start + (k - 1) / size;
            let pos = (k - 1) % size;
            let i = pos / d;
            let num = if b % 2 == 0 {
                10 * b + i
            } else {
                10 * b + 9 - i
            };
            let digit_index = (pos % d) as usize;
            let mut digits = Vec::new();
            let mut x = num;
            while x > 0 {
                digits.push((x % 10) as i32);
                x /= 10;
            }
            digits.reverse();
            return digits[digit_index];
        }
        k -= cnt * size;
        d += 1;
        start *= 10;
    }
}

fn pow10(e: i64) -> i64 {
    let mut r = 1i64;
    for _ in 0..e {
        r *= 10;
    }
    r
}

fn main() {
    println!("{}", kth_digit(15));
}

#[cfg(test)]
mod tests {
    use super::kth_digit;

    #[test]
    fn example1() {
        assert_eq!(kth_digit(4), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(kth_digit(15), 7);
    }

    #[test]
    fn example3() {
        assert_eq!(kth_digit(11), 9);
    }
}
