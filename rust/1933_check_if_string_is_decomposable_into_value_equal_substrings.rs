/// LeetCode #1933 - Check if String Is Decomposable Into Value-Equal Substrings
fn is_decomposable(s: String) -> bool {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut i = 0usize;
    let mut cnt2 = 0;
    while i < n {
        let mut j = i;
        while j < n && bytes[j] == bytes[i] {
            j += 1;
        }
        let len = j - i;
        if len % 3 == 1 {
            return false;
        }
        if len % 3 == 2 {
            cnt2 += 1;
            if cnt2 > 1 {
                return false;
            }
        }
        i = j;
    }
    cnt2 == 1
}

fn main() {
    println!("{}", is_decomposable("000111000".into()));
}

#[cfg(test)]
mod tests {
    use super::is_decomposable;

    #[test]
    fn example_one() {
        assert!(!is_decomposable("000111000".into()));
    }

    #[test]
    fn example_two() {
        assert!(is_decomposable("00011111222".into()));
    }

    #[test]
    fn example_three() {
        assert!(!is_decomposable("01110002223300".into()));
    }
}
