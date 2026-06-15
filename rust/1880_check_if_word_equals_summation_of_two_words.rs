/// LeetCode #1880 - Check if Word Equals Summation of Two Words
fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
    fn f(s: &str) -> i64 {
        let mut ans = 0i64;
        for c in s.bytes() {
            ans = ans * 10 + (c - b'a') as i64;
        }
        ans
    }
    f(&first_word) + f(&second_word) == f(&target_word)
}

fn main() {
    println!(
        "{}",
        is_sum_equal("acb".into(), "cba".into(), "cdb".into())
    );
}

#[cfg(test)]
mod tests {
    use super::is_sum_equal;

    #[test]
    fn example_one() {
        assert!(is_sum_equal("acb".into(), "cba".into(), "cdb".into()));
    }
}
