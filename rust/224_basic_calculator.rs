/// LeetCode #224 - Basic Calculator
fn calculate(s: String) -> i32 {
    let b = s.as_bytes();
    let mut i = 0usize;

    fn expr(b: &[u8], i: &mut usize) -> i32 {
        let mut sign = 1i32;
        let mut sum = 0i32;
        while *i < b.len() {
            match b[*i] {
                b' ' => *i += 1,
                b'+' => {
                    *i += 1;
                    sign = 1;
                }
                b'-' => {
                    *i += 1;
                    sign = -1;
                }
                b'(' => {
                    *i += 1;
                    let v = expr(b, i);
                    sum += sign * v;
                    sign = 1;
                }
                b')' => {
                    *i += 1;
                    return sum;
                }
                _ => {
                    let mut n = 0i32;
                    while *i < b.len() && b[*i].is_ascii_digit() {
                        n = n * 10 + (b[*i] - b'0') as i32;
                        *i += 1;
                    }
                    sum += sign * n;
                    sign = 1;
                }
            }
        }
        sum
    }

    expr(b, &mut i)
}

fn main() {
    println!("{}", calculate("1 + 1".into()));
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn example_one() {
        assert_eq!(calculate("1 + 1".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(calculate(" 2-1 + 2 ".into()), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(calculate("(1+(4+5+2)-3)+(6+8)".into()), 23);
    }
}
