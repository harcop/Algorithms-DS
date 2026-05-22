/// LeetCode #1209 - Remove All Adjacent Duplicates in String II
fn remove_duplicates(s: String, k: i32) -> String {
    let k = k as usize;
    let mut st: Vec<(char, usize)> = Vec::new();
    for c in s.chars() {
        if let Some((ch, cnt)) = st.last_mut() {
            if *ch == c {
                *cnt += 1;
                if *cnt == k {
                    st.pop();
                }
                continue;
            }
        }
        st.push((c, 1));
    }
    st.into_iter()
        .flat_map(|(c, cnt)| std::iter::repeat_n(c, cnt))
        .collect()
}

fn main() {
    println!("{}", remove_duplicates("deeedbbcccbdaa".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::remove_duplicates;

    #[test]
    fn example_one() {
        assert_eq!(remove_duplicates("deeedbbcccbdaa".into(), 3), "aa");
    }

    #[test]
    fn example_two() {
        assert_eq!(remove_duplicates("pbbcggttciiippooaais".into(), 2), "ps");
    }
}
