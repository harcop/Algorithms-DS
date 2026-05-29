/// LeetCode #1544 - Make The String Great
fn make_good(s: String) -> String {
    let mut st: Vec<char> = Vec::new();
    for c in s.chars() {
        if let Some(&last) = st.last() {
            if (last as i32 - c as i32).abs() == 32 {
                st.pop();
                continue;
            }
        }
        st.push(c);
    }
    st.into_iter().collect()
}

fn main() {
    println!("{}", make_good("leEeetcode".into()));
}

#[cfg(test)]
mod tests {
    use super::make_good;

    #[test]
    fn example_one() {
        assert_eq!(make_good("leEeetcode".into()), "leetcode");
    }

    #[test]
    fn example_two() {
        assert_eq!(make_good("abBAcC".into()), "");
    }
}
