/// LeetCode #522 - Longest Uncommon Subsequence II
fn is_subseq(a: &str, b: &str) -> bool {
    let mut it = b.chars();
    a.chars().all(|c| it.any(|d| d == c))
}

fn find_lu_length(strs: Vec<String>) -> i32 {
    let mut strs = strs;
    strs.sort_by_key(|s| std::cmp::Reverse(s.len()));
    for i in 0..strs.len() {
        let uncommon = (0..strs.len()).all(|j| i == j || !is_subseq(&strs[i], &strs[j]));
        if uncommon {
            return strs[i].len() as i32;
        }
    }
    -1
}

fn main() {
    let strs = vec!["aba".into(), "cdc".into(), "eae".into()];
    println!("{}", find_lu_length(strs));
}

#[cfg(test)]
mod tests {
    use super::find_lu_length;

    #[test]
    fn example_one() {
        let strs = vec!["aba".into(), "cdc".into(), "eae".into()];
        assert_eq!(find_lu_length(strs), 3);
    }

    #[test]
    fn example_two() {
        let strs = vec!["aaa".into(), "aaa".into(), "aa".into()];
        assert_eq!(find_lu_length(strs), -1);
    }
}
