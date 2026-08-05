/// LeetCode #3029 - Minimum Time to Revert Word to Initial State I
fn minimum_time_to_revert(word: String, k: i32) -> i32 {
    let k = k as usize;
    let n = word.len();
    let bytes = word.as_bytes();

    for i in (k..n).step_by(k) {
        if &bytes[i..] == &bytes[..n - i] {
            return (i / k) as i32;
        }
    }
    ((n + k - 1) / k) as i32
}

fn main() {
    println!("{}", minimum_time_to_revert("abacaba".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::minimum_time_to_revert;

    #[test]
    fn example1() {
        assert_eq!(minimum_time_to_revert("abacaba".into(), 3), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_time_to_revert("abacaba".into(), 4), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_time_to_revert("abcbabcd".into(), 2), 4);
    }
}
