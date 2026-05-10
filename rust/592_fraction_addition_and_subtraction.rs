/// LeetCode #592 - Fraction Addition and Subtraction
fn fraction_addition(expression: String) -> String {
    let mut num = 0i64;
    let mut den = 1i64;
    let mut i = 0usize;
    let b = expression.as_bytes();
    while i < b.len() {
        let mut sign = 1i64;
        if b[i] == b'+' || b[i] == b'-' {
            if b[i] == b'-' {
                sign = -1;
            }
            i += 1;
        } else if i == 0 && b[i] == b'-' {
            sign = -1;
            i += 1;
        }
        let mut n = 0i64;
        while i < b.len() && b[i].is_ascii_digit() {
            n = n * 10 + (b[i] - b'0') as i64;
            i += 1;
        }
        i += 1;
        let mut d = 0i64;
        while i < b.len() && b[i].is_ascii_digit() {
            d = d * 10 + (b[i] - b'0') as i64;
            i += 1;
        }
        n *= sign;
        num = num * d + n * den;
        den *= d;
        let g = gcd(num.abs(), den);
        num /= g;
        den /= g;
    }
    format!("{}/{}", num, den)
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn main() {
    println!("{}", fraction_addition("-1/2+1/2".into()));
}

#[cfg(test)]
mod tests {
    use super::fraction_addition;

    #[test]
    fn example_one() {
        assert_eq!(fraction_addition("-1/2+1/2".into()), "0/1");
    }

    #[test]
    fn example_two() {
        assert_eq!(fraction_addition("1/2+1/3+1/6".into()), "1/1");
    }
}
