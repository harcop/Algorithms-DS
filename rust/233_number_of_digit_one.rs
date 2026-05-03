/// LeetCode #233 - Number of Digit One
fn count_digit_one(n: i32) -> i32 {
    let mut ans = 0i64;
    let mut i = 1i64;
    let n = n as i64;
    while i <= n {
        let divider = i * 10;
        ans += n / divider * i + (n % divider - i + 1).clamp(0, i);
        i *= 10;
    }
    ans as i32
}

fn main() {
    println!("{}", count_digit_one(13));
}

#[cfg(test)]
mod tests {
    use super::count_digit_one;

    #[test]
    fn example_one() {
        assert_eq!(count_digit_one(13), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_digit_one(0), 0);
    }
}
