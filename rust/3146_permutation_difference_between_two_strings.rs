/// LeetCode #3146 - Permutation Difference between Two Strings
fn find_permutation_difference(s: String, t: String) -> i32 {
    let mut pos = [0usize; 26];
    for (i, c) in s.bytes().enumerate() {
        pos[(c - b'a') as usize] = i;
    }
    t.bytes()
        .enumerate()
        .map(|(i, c)| (pos[(c - b'a') as usize] as i32 - i as i32).abs())
        .sum()
}

fn main() {
    println!("{}", find_permutation_difference("abc".into(), "bac".into()));
}

#[cfg(test)]
mod tests {
    use super::find_permutation_difference;

    #[test]
    fn example1() {
        assert_eq!(find_permutation_difference("abc".into(), "bac".into()), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_permutation_difference("abcde".into(), "edbac".into()),
            12
        );
    }
}
