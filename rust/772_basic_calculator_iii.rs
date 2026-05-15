/// LeetCode #772 - Basic Calculator III
fn calculate(s: String) -> i32 {
    let b = s.as_bytes();
    let mut i = 0usize;
    fn expr(b: &[u8], i: &mut usize) -> i32 {
        let mut val = term(b, i);
        while *i < b.len() && (b[*i] == b'+' || b[*i] == b'-') {
            let op = b[*i];
            *i += 1;
            let t = term(b, i);
            val = if op == b'+' { val + t } else { val - t };
        }
        val
    }
    fn term(b: &[u8], i: &mut usize) -> i32 {
        let mut val = factor(b, i);
        while *i < b.len() && (b[*i] == b'*' || b[*i] == b'/') {
            let op = b[*i];
            *i += 1;
            let f = factor(b, i);
            val = if op == b'*' { val * f } else { val / f };
        }
        val
    }
    fn factor(b: &[u8], i: &mut usize) -> i32 {
        if b[*i] == b'(' {
            *i += 1;
            let v = expr(b, i);
            *i += 1;
            v
        } else {
            let mut v = 0i32;
            while *i < b.len() && b[*i].is_ascii_digit() {
                v = v * 10 + (b[*i] - b'0') as i32;
                *i += 1;
            }
            v
        }
    }
    expr(b, &mut i)
}

fn main() {
    println!("{}", calculate("2*(6-4)+8".into()));
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn example_one() {
        assert_eq!(calculate("1+1".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(calculate("6-4/2".into()), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(calculate("2*(6-4)+8".into()), 12);
    }
}
