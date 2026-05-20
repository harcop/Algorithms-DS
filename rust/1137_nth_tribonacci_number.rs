/// LeetCode #1137 - N-th Tribonacci Number
fn tribonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n <= 2 {
        return 1;
    }
    let mut a = 0i64;
    let mut b = 1i64;
    let mut c = 1i64;
    for _ in 3..=n {
        let d = a + b + c;
        a = b;
        b = c;
        c = d;
    }
    c as i32
}

fn main() {
    println!("{}", tribonacci(4));
}

#[cfg(test)]
mod tests {
    use super::tribonacci;

    #[test]
    fn example_one() {
        assert_eq!(tribonacci(4), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(tribonacci(25), 1389537);
    }
}
