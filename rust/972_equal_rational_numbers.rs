/// LeetCode #972 - Equal Rational Numbers
fn is_rational_equal(s1: String, s2: String) -> bool {
    to_fraction(&s1) == to_fraction(&s2)
}

fn to_fraction(s: &str) -> (i128, i128) {
    if let Some(lp) = s.find('(') {
        let left = &s[..lp];
        let rep = &s[lp + 1..s.len() - 1];
        let dot = left.find('.').unwrap_or(left.len());
        let int_part: i128 = if dot == 0 {
            0
        } else {
            left[..dot].parse().unwrap_or(0)
        };
        let frac = if dot < left.len() { &left[dot + 1..] } else { "" };
        let non_val: i128 = if frac.is_empty() {
            0
        } else {
            frac.parse().unwrap()
        };
        let rep_val: i128 = rep.parse().unwrap();
        let non_len = frac.len();
        let rep_len = rep.len();
        let num = non_val * 10i128.pow(rep_len as u32) + rep_val - non_val;
        let den = (10i128.pow(rep_len as u32) - 1) * 10i128.pow(non_len as u32);
        reduce(int_part * den + num, den)
    } else if let Some(dot) = s.find('.') {
        let int_part: i128 = s[..dot].parse().unwrap_or(0);
        let frac = &s[dot + 1..];
        if frac.is_empty() {
            return (int_part, 1);
        }
        let frac_val: i128 = frac.parse().unwrap();
        let scale = 10i128.pow(frac.len() as u32);
        reduce(int_part * scale + frac_val, scale)
    } else {
        (s.parse().unwrap(), 1)
    }
}

fn reduce(num: i128, den: i128) -> (i128, i128) {
    let g = gcd(num.abs(), den.abs());
    let sign = if den < 0 { -1 } else { 1 };
    (sign * num / g, sign * den / g)
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn main() {
    println!("{}", is_rational_equal("0.(52)".into(), "0.5(25)".into()));
}

#[cfg(test)]
mod tests {
    use super::is_rational_equal;

    #[test]
    fn example_one() {
        assert!(is_rational_equal("0.(52)".into(), "0.5(25)".into()));
    }

    #[test]
    fn example_two() {
        assert!(is_rational_equal("0.1666(6)".into(), "0.166(66)".into()));
    }

    #[test]
    fn example_three() {
        assert!(is_rational_equal("0.9(9)".into(), "1.".into()));
    }
}
