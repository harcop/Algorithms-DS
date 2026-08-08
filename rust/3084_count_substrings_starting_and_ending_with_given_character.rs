/// LeetCode #3084 - Count Substrings Starting and Ending with Given Character
fn count_substrings(s: String, c: char) -> i64 {
    let cnt = s.chars().filter(|&ch| ch == c).count() as i64;
    cnt * (cnt + 1) / 2
}

fn main() {
    println!("{}", count_substrings("abada".into(), 'a'));
}

#[cfg(test)]
mod tests {
    use super::count_substrings;

    #[test]
    fn example1() {
        assert_eq!(count_substrings("abada".into(), 'a'), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(count_substrings("zzz".into(), 'z'), 6);
    }
}
