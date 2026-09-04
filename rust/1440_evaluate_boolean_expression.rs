/// LeetCode #1440 - Evaluate Boolean Expression (SQL; Rust analogue)
use std::collections::HashMap;

fn evaluate_boolean(
    variables: Vec<(String, i32)>,
    expressions: Vec<(String, String, String)>,
) -> Vec<(String, String, String, String)> {
    let vals: HashMap<String, i32> = variables.into_iter().collect();
    expressions
        .into_iter()
        .map(|(left, op, right)| {
            let lv = vals[&left];
            let rv = vals[&right];
            let ok = match op.as_str() {
                ">" => lv > rv,
                "<" => lv < rv,
                "=" => lv == rv,
                _ => false,
            };
            (
                left,
                op,
                right,
                if ok { "true".into() } else { "false".into() },
            )
        })
        .collect()
}

fn main() {
    println!("{:?}", evaluate_boolean(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::evaluate_boolean;

    #[test]
    fn example() {
        let variables = vec![("x".into(), 66), ("y".into(), 77)];
        let expressions = vec![
            ("x".into(), ">".into(), "y".into()),
            ("x".into(), "<".into(), "y".into()),
            ("x".into(), "=".into(), "y".into()),
            ("y".into(), ">".into(), "x".into()),
            ("y".into(), "<".into(), "x".into()),
            ("x".into(), "=".into(), "x".into()),
        ];
        assert_eq!(
            evaluate_boolean(variables, expressions),
            vec![
                ("x".into(), ">".into(), "y".into(), "false".into()),
                ("x".into(), "<".into(), "y".into(), "true".into()),
                ("x".into(), "=".into(), "y".into(), "false".into()),
                ("y".into(), ">".into(), "x".into(), "true".into()),
                ("y".into(), "<".into(), "x".into(), "false".into()),
                ("x".into(), "=".into(), "x".into(), "true".into()),
            ]
        );
    }
}
