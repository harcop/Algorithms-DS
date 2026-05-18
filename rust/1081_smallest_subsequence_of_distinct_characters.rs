/// LeetCode #1081 - Smallest Subsequence of Distinct Characters
fn smallest_subsequence(s: String) -> String {
    let mut last = [0usize; 26];
    for (i, c) in s.bytes().enumerate() {
        last[(c - b'a') as usize] = i;
    }
    let mut seen = [false; 26];
    let mut st: Vec<u8> = Vec::new();
    for (i, c) in s.bytes().enumerate() {
        let ci = (c - b'a') as usize;
        if seen[ci] {
            continue;
        }
        while let Some(&top) = st.last() {
            let ti = (top - b'a') as usize;
            if top > c && last[ti] > i {
                seen[ti] = false;
                st.pop();
            } else {
                break;
            }
        }
        seen[ci] = true;
        st.push(c);
    }
    String::from_utf8(st).unwrap()
}

fn main() {
    println!("{}", smallest_subsequence("cdadabcc".into()));
}

#[cfg(test)]
mod tests {
    use super::smallest_subsequence;

    #[test]
    fn example_one() {
        assert_eq!(smallest_subsequence("cdadabcc".into()), "adbc");
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_subsequence("abcd".into()), "abcd");
    }
}
