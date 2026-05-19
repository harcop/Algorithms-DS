/// LeetCode #1106 - Parsing A Boolean Expression
fn parse_bool_expr(expression: String) -> bool {
    let mut st: Vec<char> = Vec::new();
    for c in expression.chars() {
        if c == ')' {
            let mut vals = Vec::new();
            while let Some(&top) = st.last() {
                if top == '(' {
                    st.pop();
                    break;
                }
                vals.push(st.pop().unwrap());
            }
            let op = st.pop().unwrap();
            let v = match op {
                '!' => vals[0] == 't',
                '&' => vals.iter().all(|&x| x == 't'),
                '|' => vals.iter().any(|&x| x == 't'),
                _ => unreachable!(),
            };
            st.push(if v { 't' } else { 'f' });
        } else if c != ',' {
            st.push(c);
        }
    }
    st[0] == 't'
}

fn main() {
    println!("{}", parse_bool_expr("!(f)".into()));
}

#[cfg(test)]
mod tests {
    use super::parse_bool_expr;

    #[test]
    fn example_one() {
        assert!(!parse_bool_expr("!(f)".into()));
    }

    #[test]
    fn example_two() {
        assert!(parse_bool_expr("|(f,t)".into()));
    }

    #[test]
    fn example_three() {
        assert!(parse_bool_expr("&(t,f)".into()) == false);
    }
}
