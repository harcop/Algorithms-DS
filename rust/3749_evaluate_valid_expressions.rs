/// LeetCode #3749 - Evaluate Valid Expressions (premium)
fn evaluate_expression(expression: String) -> i64 {
    parse(expression.as_bytes(), 0).0
}

fn parse(s: &[u8], mut i: usize) -> (i64, usize) {
    if s[i] == b'-' || s[i].is_ascii_digit() {
        let start = i;
        if s[i] == b'-' {
            i += 1;
        }
        while i < s.len() && s[i].is_ascii_digit() {
            i += 1;
        }
        let num = std::str::from_utf8(&s[start..i]).unwrap().parse::<i64>().unwrap();
        return (num, i);
    }
    let start = i;
    while s[i] != b'(' {
        i += 1;
    }
    let op = &s[start..i];
    i += 1;
    let (val1, ni) = parse(s, i);
    i = ni + 1;
    let (val2, ni) = parse(s, i);
    i = ni + 1;
    let res = match op {
        b"add" => val1 + val2,
        b"sub" => val1 - val2,
        b"mul" => val1 * val2,
        b"div" => val1 / val2,
        _ => 0,
    };
    (res, i)
}

fn main() {
    println!("{}", evaluate_expression("add(2,3)".into()));
}

#[cfg(test)]
mod tests {
    use super::evaluate_expression;

    #[test]
    fn example1() {
        assert_eq!(evaluate_expression("add(2,3)".into()), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(evaluate_expression("-42".into()), -42);
    }

    #[test]
    fn example3() {
        assert_eq!(
            evaluate_expression("div(mul(4,sub(9,5)),add(1,1))".into()),
            8
        );
    }
}
