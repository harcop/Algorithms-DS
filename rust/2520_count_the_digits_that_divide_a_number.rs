/// LeetCode #2520 - Count the Digits That Divide a Number
fn count_digits(num: i32) -> i32 {
    let mut ans = 0;
    let mut x = num;
    while x > 0 {
        if num % (x % 10) == 0 {
            ans += 1;
        }
        x /= 10;
    }
    ans
}

fn main() {
    println!("{}", count_digits(121));
}

#[cfg(test)]
mod tests {
    use super::count_digits;

    #[test]
    fn example_one() {
        assert_eq!(count_digits(7), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_digits(121), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_digits(1248), 4);
    }
}
