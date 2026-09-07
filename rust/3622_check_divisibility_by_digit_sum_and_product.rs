/// LeetCode #3622 - Check Divisibility by Digit Sum and Product
fn check_divisibility(n: i32) -> bool {
    let mut s = 0;
    let mut p = 1;
    let mut x = n;
    while x != 0 {
        let v = x % 10;
        x /= 10;
        s += v;
        p *= v;
    }
    n % (s + p) == 0
}

fn main() {
    println!("{}", check_divisibility(99));
}

#[cfg(test)]
mod tests {
    use super::check_divisibility;

    #[test]
    fn example1() {
        assert_eq!(check_divisibility(99), true);
    }

    #[test]
    fn example2() {
        assert_eq!(check_divisibility(23), false);
    }
}
