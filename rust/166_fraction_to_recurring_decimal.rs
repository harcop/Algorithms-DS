/// LeetCode #166 - Fraction to Recurring Decimal
use std::collections::HashMap;

fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    if numerator == 0 {
        return "0".into();
    }
    let mut out = String::new();
    let neg = (numerator as i64) * (denominator as i64) < 0;
    if neg {
        out.push('-');
    }
    let mut n = (numerator as i64).abs();
    let d = (denominator as i64).abs();
    out.push_str(&(n / d).to_string());
    n %= d;
    if n == 0 {
        return out;
    }
    out.push('.');
    let mut pos: HashMap<i64, usize> = HashMap::new();
    while n != 0 {
        if let Some(&i) = pos.get(&n) {
            out.insert(i, '(');
            out.push(')');
            return out;
        }
        pos.insert(n, out.len());
        n *= 10;
        out.push_str(&(n / d).to_string());
        n %= d;
    }
    out
}

fn main() {
    println!("{}", fraction_to_decimal(4, 333));
}

#[cfg(test)]
mod tests {
    use super::fraction_to_decimal;

    #[test]
    fn example_one() {
        assert_eq!(fraction_to_decimal(1, 2), "0.5");
    }

    #[test]
    fn example_two() {
        assert_eq!(fraction_to_decimal(2, 1), "2");
    }

    #[test]
    fn example_three() {
        assert_eq!(fraction_to_decimal(4, 333), "0.(012)");
    }
}
