/// LeetCode #357 - Count Numbers with Unique Digits
fn count_numbers_with_unique_digits(n: i32) -> i32 {
    if n == 0 { return 1; }
    let mut ans = 10;
    let mut cur = 9;
    let mut avail = 9;
    let mut k = 2;
    while k <= n && avail > 0 {
        cur *= avail;
        ans += cur;
        avail -= 1;
        k += 1;
    }
    ans
}

fn main() {
    println!("{}", count_numbers_with_unique_digits(2));
}

#[cfg(test)]
mod tests {
    use super::count_numbers_with_unique_digits;

    #[test]
    fn example_one() {
        assert_eq!(count_numbers_with_unique_digits(2), 91);
    }
}
