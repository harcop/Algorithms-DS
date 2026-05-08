/// LeetCode #509 - Fibonacci Number
fn fib(n: i32) -> i32 {
    let n = n as usize;
    if n < 2 {
        return n as i32;
    }
    let (mut a, mut b) = (0i32, 1i32);
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

fn main() {
    println!("{}", fib(4));
}

#[cfg(test)]
mod tests {
    use super::fib;

    #[test]
    fn example_one() {
        assert_eq!(fib(2), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(fib(3), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(fib(4), 3);
    }
}
