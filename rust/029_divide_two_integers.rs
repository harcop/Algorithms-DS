/// LeetCode #29 - Divide Two Integers
fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == i32::MIN && divisor == -1 {
        return i32::MAX;
    }

    let negative = (dividend < 0) ^ (divisor < 0);
    let mut a = (dividend as i64).abs();
    let b = (divisor as i64).abs();
    let mut result = 0i64;

    while a >= b {
        let mut temp = b;
        let mut multiple = 1i64;
        while a >= (temp << 1) {
            temp <<= 1;
            multiple <<= 1;
        }
        a -= temp;
        result += multiple;
    }

    let result = if negative { -result } else { result };
    result as i32
}

fn main() {
    println!("{}", divide(10, 3));
}

#[cfg(test)]
mod tests {
    use super::divide;

    #[test]
    fn example_one() {
        assert_eq!(divide(10, 3), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(divide(7, -3), -2);
    }

    #[test]
    fn overflow_case() {
        assert_eq!(divide(i32::MIN, -1), i32::MAX);
    }
}
