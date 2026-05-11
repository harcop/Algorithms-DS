/// LeetCode #650 - 2 Keys Keyboard
fn min_steps(mut n: i32) -> i32 {
    let mut ans = 0i32;
    let mut d = 2i32;
    while n > 1 {
        while n % d == 0 {
            ans += d;
            n /= d;
        }
        d += 1;
    }
    ans
}

fn main() {
    println!("{}", min_steps(3));
}

#[cfg(test)]
mod tests {
    use super::min_steps;

    #[test]
    fn example_one() {
        assert_eq!(min_steps(3), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_steps(1), 0);
    }
}
