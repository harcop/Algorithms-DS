/// LeetCode #479 - Largest Palindrome Product
fn largest_palindrome(n: i32) -> i32 {
    if n == 1 {
        return 9;
    }
    let max = 10i64.pow(n as u32) - 1;
    let min = 10i64.pow((n as u32) - 1);
    for a in (min..=max).rev() {
        let s = a.to_string();
        let pal: i64 = format!("{}{}", s, s.chars().rev().collect::<String>())
            .parse()
            .unwrap();
        let mut x = max;
        while x * x >= pal {
            if pal % x == 0 {
                let y = pal / x;
                if y >= min && y <= max {
                    return (pal % 1337) as i32;
                }
            }
            x -= 1;
        }
    }
    0
}

fn main() {
    println!("{}", largest_palindrome(2));
}

#[cfg(test)]
mod tests {
    use super::largest_palindrome;

    #[test]
    fn example_one() {
        assert_eq!(largest_palindrome(2), 987);
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_palindrome(1), 9);
    }
}
