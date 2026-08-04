/// LeetCode #2957 - Remove Adjacent Almost-Equal Characters
fn remove_almost_equal_characters(word: String) -> i32 {
    let bytes = word.as_bytes();
    let n = bytes.len();
    let mut ans = 0;
    let mut i = 1;
    while i < n {
        if (bytes[i] as i32 - bytes[i - 1] as i32).abs() < 2 {
            ans += 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", remove_almost_equal_characters("aaaaa".into()));
}

#[cfg(test)]
mod tests {
    use super::remove_almost_equal_characters;

    #[test]
    fn example_one() {
        assert_eq!(remove_almost_equal_characters("aaaaa".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(remove_almost_equal_characters("abddez".into()), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(remove_almost_equal_characters("zyxyxyz".into()), 3);
    }
}
