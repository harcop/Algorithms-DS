/// LeetCode #3561 - Resulting String After Adjacent Removals
fn resulting_string(s: String) -> String {
    let mut stk = Vec::new();
    for c in s.chars() {
        if let Some(&top) = stk.last() {
            let d = (top as i32 - c as i32).abs();
            if d == 1 || d == 25 {
                stk.pop();
                continue;
            }
        }
        stk.push(c);
    }
    stk.into_iter().collect()
}

fn main() {
    println!("{}", resulting_string("abc".into()));
}

#[cfg(test)]
mod tests {
    use super::resulting_string;

    #[test]
    fn example1() {
        assert_eq!(resulting_string("abc".into()), "c");
    }

    #[test]
    fn example2() {
        assert_eq!(resulting_string("adcb".into()), "");
    }

    #[test]
    fn example3() {
        assert_eq!(resulting_string("zadb".into()), "db");
    }
}
