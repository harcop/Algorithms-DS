/// LeetCode #383 - Ransom Note
fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut cnt = [0i32; 26];
    for b in magazine.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }
    for b in ransom_note.bytes() {
        let i = (b - b'a') as usize;
        cnt[i] -= 1;
        if cnt[i] < 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", can_construct("aa".into(), "aab".into()));
}

#[cfg(test)]
mod tests {
    use super::can_construct;

    #[test]
    fn example_one() {
        assert!(!can_construct("a".into(), "b".into()));
    }

    #[test]
    fn example_two() {
        assert!(can_construct("aa".into(), "aab".into()));
    }
}
