/// LeetCode #3614 - Process String with Special Operations II
fn process_str(s: String, k: i64) -> char {
    let mut m = 0i64;
    for c in s.chars() {
        if c == '*' {
            m = (m - 1).max(0);
        } else if c == '#' {
            m <<= 1;
        } else if c != '%' {
            m += 1;
        }
    }
    if k >= m {
        return '.';
    }
    let mut k = k;
    for c in s.chars().rev() {
        if c == '*' {
            m += 1;
        } else if c == '#' {
            m /= 2;
            if k >= m {
                k -= m;
            }
        } else if c == '%' {
            k = m - 1 - k;
        } else {
            m -= 1;
            if k == m {
                return c;
            }
        }
    }
    '.'
}

fn main() {
    println!("{}", process_str("a#b%*".into(), 1));
}

#[cfg(test)]
mod tests {
    use super::process_str;

    #[test]
    fn example1() {
        assert_eq!(process_str("a#b%*".into(), 1), 'a');
    }

    #[test]
    fn example2() {
        assert_eq!(process_str("cd%#*#".into(), 3), 'd');
    }

    #[test]
    fn example3() {
        assert_eq!(process_str("z*#".into(), 0), '.');
    }
}
