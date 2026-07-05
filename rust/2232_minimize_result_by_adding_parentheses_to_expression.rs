/// LeetCode #2232 - Minimize Result by Adding Parentheses to Expression
fn minimize_result(expression: String) -> String {
    let plus = expression.find('+').unwrap();
    let l = &expression[..plus];
    let r = &expression[plus + 1..];
    let m = l.len();
    let n = r.len();
    let mut mi = i64::MAX;
    let mut ans = String::new();

    for i in 0..m {
        for j in 0..n {
            let a = if i == 0 {
                1
            } else {
                l[..i].parse::<i64>().unwrap()
            };
            let b: i64 = l[i..].parse().unwrap();
            let c: i64 = r[..=j].parse().unwrap();
            let d = if j == n - 1 {
                1
            } else {
                r[j + 1..].parse::<i64>().unwrap()
            };
            let val = a * (b + c) * d;
            if val < mi {
                mi = val;
                ans = format!(
                    "{}({}+{}){}",
                    &l[..i],
                    &l[i..],
                    &r[..=j],
                    &r[j + 1..]
                );
            }
        }
    }

    ans
}

fn main() {
    println!("{}", minimize_result("12+34".into()));
}

#[cfg(test)]
mod tests {
    use super::minimize_result;

    #[test]
    fn example_one() {
        assert_eq!(minimize_result("12+34".into()), "1(2+3)4");
    }

    #[test]
    fn example_three() {
        assert_eq!(minimize_result("999+999".into()), "(999+999)");
    }
}
