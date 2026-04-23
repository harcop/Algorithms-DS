/// LeetCode #22 - Generate Parentheses
fn generate_parenthesis(n: i32) -> Vec<String> {
    fn backtrack(open: i32, close: i32, current: &mut String, out: &mut Vec<String>) {
        if open == 0 && close == 0 {
            out.push(current.clone());
            return;
        }

        if open > 0 {
            current.push('(');
            backtrack(open - 1, close, current, out);
            current.pop();
        }
        if close > open {
            current.push(')');
            backtrack(open, close - 1, current, out);
            current.pop();
        }
    }

    let mut out = Vec::new();
    let mut current = String::new();
    backtrack(n, n, &mut current, &mut out);
    out
}

fn main() {
    println!("{:?}", generate_parenthesis(3));
}

#[cfg(test)]
mod tests {
    use super::generate_parenthesis;

    #[test]
    fn example_one() {
        let mut got = generate_parenthesis(3);
        got.sort();
        let mut expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        expected.sort();
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        assert_eq!(generate_parenthesis(1), vec!["()"]);
    }
}
