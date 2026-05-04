/// LeetCode #258 - Add Digits
fn add_digits(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    let r = num % 9;
    if r == 0 {
        9
    } else {
        r
    }
}

fn main() {
    println!("{}", add_digits(38));
}

#[cfg(test)]
mod tests {
    use super::add_digits;

    #[test]
    fn example_one() {
        assert_eq!(add_digits(38), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(add_digits(0), 0);
    }
}
