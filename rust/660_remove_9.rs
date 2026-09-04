/// LeetCode #660 - Remove 9
fn new_integer(n: i32) -> i32 {
    let mut n = n;
    let mut ans = 0i64;
    let mut base = 1i64;
    while n > 0 {
        ans += (n % 9) as i64 * base;
        n /= 9;
        base *= 10;
    }
    ans as i32
}

fn main() {
    println!("{}", new_integer(9));
}

#[cfg(test)]
mod tests {
    use super::new_integer;

    #[test]
    fn example_one() {
        assert_eq!(new_integer(9), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(new_integer(8), 8);
    }
}
