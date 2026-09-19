/// LeetCode #3813 - Vowel-Consonant Score
fn vowel_consonant_score(s: String) -> i32 {
    let mut v = 0;
    let mut c = 0;
    for ch in s.bytes() {
        if ch.is_ascii_alphabetic() {
            c += 1;
            if matches!(ch, b'a' | b'e' | b'i' | b'o' | b'u') {
                v += 1;
            }
        }
    }
    c -= v;
    if c == 0 {
        0
    } else {
        v / c
    }
}

fn main() {
    println!("{}", vowel_consonant_score("cooear".into()));
}

#[cfg(test)]
mod tests {
    use super::vowel_consonant_score;

    #[test]
    fn example1() {
        assert_eq!(vowel_consonant_score("cooear".into()), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(vowel_consonant_score("axeyizou".into()), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(vowel_consonant_score("au 123".into()), 0);
    }
}
