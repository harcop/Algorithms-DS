/// LeetCode #3945 - Digit Frequency Score
fn digit_frequency_score(mut n: i32) -> i32 {
    let mut ans = 0;
    while n > 0 {
        ans += n % 10;
        n /= 10;
    }
    ans
}

fn main() {
    println!("{}", digit_frequency_score(122));
}

#[cfg(test)]
mod tests {
    use super::digit_frequency_score;

    #[test]
    fn example1() {
        assert_eq!(digit_frequency_score(122), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(digit_frequency_score(101), 2);
    }
}
