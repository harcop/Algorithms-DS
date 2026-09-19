/// LeetCode #3803 - Count Residue Prefixes
use std::collections::HashSet;

fn residue_prefixes(s: String) -> i32 {
    let mut st = HashSet::new();
    let mut ans = 0;
    for (i, c) in s.bytes().enumerate() {
        st.insert(c);
        if st.len() == (i + 1) % 3 {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", residue_prefixes("abc".into()));
}

#[cfg(test)]
mod tests {
    use super::residue_prefixes;

    #[test]
    fn example1() {
        assert_eq!(residue_prefixes("abc".into()), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(residue_prefixes("dd".into()), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(residue_prefixes("bob".into()), 2);
    }
}
