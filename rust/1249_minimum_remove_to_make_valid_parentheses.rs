/// LeetCode #1249 - Minimum Remove to Make Valid Parentheses
fn min_remove_to_make_valid(s: String) -> String {
    let bytes = s.as_bytes();
    let mut remove = vec![false; bytes.len()];
    let mut depth = 0i32;
    for (i, &ch) in bytes.iter().enumerate() {
        if ch == b'(' {
            depth += 1;
        } else if ch == b')' {
            if depth == 0 {
                remove[i] = true;
            } else {
                depth -= 1;
            }
        }
    }
    depth = 0;
    for i in (0..bytes.len()).rev() {
        if bytes[i] == b')' {
            depth += 1;
        } else if bytes[i] == b'(' {
            if depth == 0 {
                remove[i] = true;
            } else {
                depth -= 1;
            }
        }
    }
    bytes
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if remove[i] { None } else { Some(c as char) })
        .collect()
}

fn main() {
    println!("{}", min_remove_to_make_valid("lee(t(c)o)de)".into()));
}

#[cfg(test)]
mod tests {
    use super::min_remove_to_make_valid;

    #[test]
    fn example_one() {
        assert_eq!(
            min_remove_to_make_valid("lee(t(c)o)de)".into()),
            "lee()co()".to_string()
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_remove_to_make_valid("a)b(c)d".into()),
            "ab(c)d".to_string()
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(min_remove_to_make_valid("))(("into()), "".to_string());
    }
}
