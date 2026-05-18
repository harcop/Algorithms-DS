/// LeetCode #921 - Minimum Add to Make Parentheses Valid
fn min_add_to_make_valid(s: String) -> i32 {
    let mut open = 0i32;
    let mut add = 0i32;
    for b in s.bytes() {
        if b == b'(' {
            open += 1;
        } else if open > 0 {
            open -= 1;
        } else {
            add += 1;
        }
    }
    add + open
}

fn main() {
    println!("{}", min_add_to_make_valid("())".into()));
}

#[cfg(test)]
mod tests {
    use super::min_add_to_make_valid;

    #[test]
    fn example_one() {
        assert_eq!(min_add_to_make_valid("())".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_add_to_make_valid("(((".into()), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_add_to_make_valid("()".into()), 0);
    }

    #[test]
    fn example_four() {
        assert_eq!(min_add_to_make_valid("()))((".into()), 4);
    }
}
