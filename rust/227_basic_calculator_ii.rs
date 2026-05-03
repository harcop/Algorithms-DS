/// LeetCode #227 - Basic Calculator II
fn calculate(s: String) -> i32 {
    let mut stack = vec![];
    let mut num = 0i32;
    let mut op = b'+';
    let b = s.as_bytes();
    for (i, &ch) in b.iter().enumerate() {
        if ch.is_ascii_digit() {
            num = num * 10 + (ch - b'0') as i32;
        }
        if (!ch.is_ascii_digit() && ch != b' ') || i + 1 == b.len() {
            match op {
                b'+' => stack.push(num),
                b'-' => stack.push(-num),
                b'*' => {
                    let v = stack.pop().unwrap() * num;
                    stack.push(v);
                }
                b'/' => {
                    let v = stack.pop().unwrap() / num;
                    stack.push(v);
                }
                _ => {}
            }
            if matches!(ch, b'+' | b'-' | b'*' | b'/') {
                op = ch;
            }
            num = 0;
        }
    }
    stack.iter().sum()
}

fn main() {
    println!("{}", calculate("3+2*2".into()));
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn example_one() {
        assert_eq!(calculate("3+2*2".into()), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(calculate(" 3/2 ".into()), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(calculate(" 3+5 / 2 ".into()), 5);
    }
}
