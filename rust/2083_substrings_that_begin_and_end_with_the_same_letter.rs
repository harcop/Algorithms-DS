/// LeetCode #2083 - Substrings That Begin and End With the Same Letter
fn number_of_substrings(s: String) -> i64 {
    let mut counts = [0i64; 26];
    for b in s.bytes() {
        counts[(b - b'a') as usize] += 1;
    }
    counts.iter().map(|&count| count * (count + 1) / 2).sum()
}

fn main() {
    println!("{}", number_of_substrings("abcba".into()));
}

#[cfg(test)]
mod tests {
    use super::number_of_substrings;

    #[test]
    fn example_one() {
        assert_eq!(number_of_substrings("abcba".into()), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_substrings("abacad".into()), 9);
    }

    #[test]
    fn example_three() {
        assert_eq!(number_of_substrings("a".into()), 1);
    }
}
