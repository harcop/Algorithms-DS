/// LeetCode #736 - Parse Lisp Expression
use std::collections::HashMap;

fn split_top_level(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut depth = 0i32;
    for ch in s.chars() {
        match ch {
            '(' => {
                depth += 1;
                cur.push(ch);
            }
            ')' => {
                depth -= 1;
                cur.push(ch);
            }
            ' ' if depth == 0 => {
                if !cur.is_empty() {
                    out.push(cur);
                    cur = String::new();
                }
            }
            _ => cur.push(ch),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn evaluate(expression: String) -> i32 {
    fn eval(e: &str, scope: &HashMap<String, i32>) -> i32 {
        let e = e.trim();
        if !e.starts_with('(') {
            if e.starts_with('-') || e.chars().all(|c| c.is_ascii_digit()) {
                return e.parse().unwrap();
            }
            return *scope.get(e).unwrap();
        }
        let inner = &e[1..e.len() - 1];
        let parts = split_top_level(inner);
        match parts[0].as_str() {
            "add" => eval(&parts[1], scope) + eval(&parts[2], scope),
            "mult" => eval(&parts[1], scope) * eval(&parts[2], scope),
            "let" => {
                let mut sc = scope.clone();
                let n = parts.len();
                let mut i = 1;
                while i + 1 < n {
                    let v = eval(&parts[i + 1], &sc);
                    sc.insert(parts[i].clone(), v);
                    i += 2;
                }
                eval(&parts[n - 1], &sc)
            }
            _ => unreachable!(),
        }
    }
    eval(expression.trim(), &HashMap::new())
}

fn main() {
    println!("{}", evaluate("(add 1 2)".into()));
}

#[cfg(test)]
mod tests {
    use super::evaluate;

    #[test]
    fn example_one() {
        assert_eq!(evaluate("(add 1 2)".into()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(evaluate("(mult 3 (add 2 3))".into()), 15);
    }

    #[test]
    fn example_three() {
        assert_eq!(evaluate("(let x 2 (mult x (let x 3 x)))".into()), 6);
    }
}
