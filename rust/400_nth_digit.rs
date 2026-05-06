/// LeetCode #400 - Nth Digit
fn find_nth_digit(n: i32) -> i32 {
    let mut n = n as i64;
    let mut len = 1i64;
    let mut count = 9i64;
    let mut start = 1i64;
    while n > len * count {
        n -= len * count;
        len += 1;
        count *= 10;
        start *= 10;
    }
    let num = start + (n - 1) / len;
    let idx = (n - 1) % len;
    num.to_string().as_bytes()[idx as usize] as char as i32 - '0' as i32
}

fn main() {
    println!("{}", find_nth_digit(11));
}

#[cfg(test)]
mod tests {
    use super::find_nth_digit;

    #[test]
    fn example_one() {
        assert_eq!(find_nth_digit(3), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_nth_digit(11), 0);
    }
}
