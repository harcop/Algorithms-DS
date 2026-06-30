/// LeetCode #2180 - Count Integers With Even Digit Sum
fn count_even(num: i32) -> i32 {
    let mut ans = 0i32;
    for x in 1..=num {
        let mut y = x;
        let mut digit_sum = 0i32;
        while y > 0 {
            digit_sum += y % 10;
            y /= 10;
        }
        if digit_sum % 2 == 0 {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_even(30));
}

#[cfg(test)]
mod tests {
    use super::count_even;

    #[test]
    fn example_one() {
        assert_eq!(count_even(4), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_even(30), 14);
    }
}
