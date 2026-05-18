/// LeetCode #953 - Verifying an Alien Dictionary

fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    let rank: Vec<usize> = {
        let mut r = vec![0usize; 26];
        for (i, c) in order.bytes().enumerate() {
            r[(c - b'a') as usize] = i;
        }
        r
    };
    let less = |a: &[u8], b: &[u8]| -> bool {
        let n = a.len().min(b.len());
        for i in 0..n {
            if a[i] != b[i] {
                return rank[(a[i] - b'a') as usize] < rank[(b[i] - b'a') as usize];
            }
        }
        a.len() <= b.len()
    };
    for i in 1..words.len() {
        let (a, b) = (words[i - 1].as_bytes(), words[i].as_bytes());
        if !less(a, b) {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", is_alien_sorted(vec!["hello".into(), "leetcode".into()], "hlabcdefgijkmnopqrstuvwxyz".into()));
}

#[cfg(test)]
mod tests {
    use super::is_alien_sorted;

    #[test]
    fn example_one() {
        assert!(is_alien_sorted(
            vec!["hello".into(), "leetcode".into()],
            "hlabcdefgijkmnopqrstuvwxyz".into()
        ));
    }

    #[test]
    fn example_two() {
        assert!(!is_alien_sorted(
            vec!["word".into(), "world".into(), "row".into()],
            "worldabcefghijkmnpqstuvxyz".into()
        ));
    }

    #[test]
    fn example_three() {
        assert!(!is_alien_sorted(
            vec!["apple".into(), "app".into()],
            "abcdefghijklmnopqrstuvwxyz".into()
        ));
    }
}
