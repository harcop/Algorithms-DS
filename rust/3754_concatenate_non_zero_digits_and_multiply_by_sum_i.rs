/// LeetCode #3754 - Concatenate Non-Zero Digits and Multiply by Sum I
fn sum_and_multiply(mut n: i32) -> i64 {
    let mut p = 1i64;
    let mut x = 0i64;
    let mut s = 0i64;
    while n > 0 {
        let v = (n % 10) as i64;
        if v != 0 {
            s += v;
            x += p * v;
            p *= 10;
        }
        n /= 10;
    }
    x * s
}

fn main() {
    println!("{}", sum_and_multiply(10203004));
}

#[cfg(test)]
mod tests {
    use super::sum_and_multiply;

    #[test]
    fn example1() {
        assert_eq!(sum_and_multiply(10203004), 12340);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_and_multiply(1000), 1);
    }
}
