/// LeetCode #150 - Evaluate Reverse Polish Notation
fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut st: Vec<i32> = Vec::new();
    for t in tokens {
        match t.as_str() {
            "+" => {
                let b = st.pop().unwrap();
                let a = st.pop().unwrap();
                st.push(a + b);
            }
            "-" => {
                let b = st.pop().unwrap();
                let a = st.pop().unwrap();
                st.push(a - b);
            }
            "*" => {
                let b = st.pop().unwrap();
                let a = st.pop().unwrap();
                st.push(a * b);
            }
            "/" => {
                let b = st.pop().unwrap();
                let a = st.pop().unwrap();
                st.push(a / b);
            }
            _ => st.push(t.parse().unwrap()),
        }
    }
    st.pop().unwrap()
}

fn main() {
    println!(
        "{}",
        eval_rpn(vec![
            "2".into(),
            "1".into(),
            "+".into(),
            "3".into(),
            "*".into(),
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::eval_rpn;

    #[test]
    fn example_one() {
        assert_eq!(
            eval_rpn(vec![
                "2".into(),
                "1".into(),
                "+".into(),
                "3".into(),
                "*".into(),
            ]),
            9
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            eval_rpn(vec![
                "4".into(),
                "13".into(),
                "5".into(),
                "/".into(),
                "+".into(),
            ]),
            6
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            eval_rpn(vec![
                "10".into(),
                "6".into(),
                "9".into(),
                "3".into(),
                "+".into(),
                "-11".into(),
                "*".into(),
                "/".into(),
                "*".into(),
                "17".into(),
                "+".into(),
                "5".into(),
                "+".into(),
            ]),
            22
        );
    }
}
