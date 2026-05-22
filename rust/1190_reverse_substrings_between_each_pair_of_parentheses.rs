/// LeetCode #1190 - Reverse Substrings Between Each Pair of Parentheses
fn reverse_parentheses(s: String) -> String {
    let mut st: Vec<char> = Vec::new();
    for c in s.chars() {
        if c == ')' {
            let mut rev = Vec::new();
            while let Some(ch) = st.pop() {
                if ch == '(' {
                    break;
                }
                rev.push(ch);
            }
            st.extend(rev);
        } else {
            st.push(c);
        }
    }
    st.into_iter().collect()
}

fn main() {
    println!("{}", reverse_parentheses("(abcd)".into()));
}

#[cfg(test)]
mod tests {
    use super::reverse_parentheses;

    #[test]
    fn example_one() {
        assert_eq!(reverse_parentheses("(abcd)".into()), "dcba");
    }

    #[test]
    fn example_two() {
        assert_eq!(reverse_parentheses("(u(love)i)".into()), "iloveu");
    }

    #[test]
    fn example_three() {
        assert_eq!(reverse_parentheses("(ed(et(oc))el)".into()), "leetcode");
    }
}
