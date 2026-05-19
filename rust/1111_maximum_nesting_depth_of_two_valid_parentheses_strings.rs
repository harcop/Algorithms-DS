/// LeetCode #1111 - Maximum Nesting Depth of Two Valid Parentheses Strings
fn max_depth_two_valid_parentheses(seq: String) -> Vec<i32> {
    let b = seq.as_bytes();
    let mut group = vec![0u8; b.len()];
    let mut depth = 0i32;
    for (i, &c) in b.iter().enumerate() {
        if c == b'(' {
            depth += 1;
            group[i] = (depth % 2) as u8;
        } else {
            group[i] = (depth % 2) as u8;
            depth -= 1;
        }
    }
    let mut dep = [0i32; 2];
    depth = 0;
    for (i, &c) in b.iter().enumerate() {
        if c == b'(' {
            depth += 1;
            dep[group[i] as usize] = dep[group[i] as usize].max(depth);
        } else {
            depth -= 1;
        }
    }
    vec![dep[0], dep[1]]
}

fn main() {
    println!("{:?}", max_depth_two_valid_parentheses("(()())".into()));
}

#[cfg(test)]
mod tests {
    use super::max_depth_two_valid_parentheses;

    #[test]
    fn example_one() {
        assert_eq!(max_depth_two_valid_parentheses("(()())".into()), vec![2, 1]);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_depth_two_valid_parentheses("()()(())()".into()), vec![2, 1]);
    }
}
