/// LeetCode #2081 - Sum of k-Mirror Numbers
fn k_mirror(k: i32, n: i32) -> i64 {
    let mut found = 0;
    let mut sum = 0i64;

    for len in 1.. {
        let half_len = (len + 1) / 2;
        let start = if half_len == 1 {
            1
        } else {
            10i64.pow(half_len as u32 - 1)
        };
        let end = 10i64.pow(half_len as u32);

        for half in start..end {
            let value = make_palindrome(half, len % 2 == 1);
            if is_palindrome_in_base(value, k as i64) {
                sum += value;
                found += 1;
                if found == n {
                    return sum;
                }
            }
        }
    }

    sum
}

fn make_palindrome(half: i64, odd_len: bool) -> i64 {
    let mut value = half;
    let mut rest = if odd_len { half / 10 } else { half };
    while rest > 0 {
        value = value * 10 + rest % 10;
        rest /= 10;
    }
    value
}

fn is_palindrome_in_base(mut value: i64, base: i64) -> bool {
    let mut digits = Vec::new();
    while value > 0 {
        digits.push(value % base);
        value /= base;
    }
    digits.iter().eq(digits.iter().rev())
}

fn main() {
    println!("{}", k_mirror(2, 5));
}

#[cfg(test)]
mod tests {
    use super::k_mirror;

    #[test]
    fn example_one() {
        assert_eq!(k_mirror(2, 5), 25);
    }

    #[test]
    fn example_two() {
        assert_eq!(k_mirror(3, 7), 499);
    }

    #[test]
    fn example_three() {
        assert_eq!(k_mirror(7, 17), 20379000);
    }
}
