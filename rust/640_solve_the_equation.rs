/// LeetCode #640 - Solve the Equation
fn solve_equation(equation: String) -> String {
    fn parse_side(s: &str) -> (i32, i32) {
        let s = s.as_bytes();
        let mut i = 0;
        let mut coef = 0;
        let mut constv = 0;
        while i < s.len() {
            let mut sign = 1;
            if s[i] == b'+' {
                i += 1;
            } else if s[i] == b'-' {
                sign = -1;
                i += 1;
            }
            let mut val = 0;
            let mut has_num = false;
            while i < s.len() && s[i].is_ascii_digit() {
                has_num = true;
                val = val * 10 + (s[i] - b'0') as i32;
                i += 1;
            }
            if i < s.len() && s[i] == b'x' {
                coef += sign * if has_num { val } else { 1 };
                i += 1;
            } else {
                constv += sign * val;
            }
        }
        (coef, constv)
    }
    let (left, right) = equation.split_once('=').unwrap();
    let (lc, lv) = parse_side(left);
    let (rc, rv) = parse_side(right);
    let coef = lc - rc;
    let val = rv - lv;
    if coef == 0 {
        if val == 0 {
            "Infinite solutions".into()
        } else {
            "No solution".into()
        }
    } else {
        format!("x={}", val / coef)
    }
}

fn main() {
    println!("{}", solve_equation("x+5-3+x=6+x-2".into()));
}

#[cfg(test)]
mod tests {
    use super::solve_equation;

    #[test]
    fn example_one() {
        assert_eq!(solve_equation("x+5-3+x=6+x-2".into()), "x=2");
    }

    #[test]
    fn example_two() {
        assert_eq!(solve_equation("x=x".into()), "Infinite solutions");
    }

    #[test]
    fn example_three() {
        assert_eq!(solve_equation("2x=x".into()), "x=0");
    }
}
