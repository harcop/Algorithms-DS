/// LeetCode #917 - Reverse Only Letters
fn reverse_only_letters(s: String) -> String {
    let mut bytes: Vec<u8> = s.into_bytes();
    let mut i = 0usize;
    let mut j = bytes.len();
    while i < j {
        while i < j && !bytes[i].is_ascii_alphabetic() {
            i += 1;
        }
        while i < j && !bytes[j - 1].is_ascii_alphabetic() {
            j -= 1;
        }
        if i < j {
            bytes.swap(i, j - 1);
            i += 1;
            j -= 1;
        }
    }
    String::from_utf8(bytes).unwrap()
}

fn main() {
    println!("{}", reverse_only_letters("ab-cd".into()));
}

#[cfg(test)]
mod tests {
    use super::reverse_only_letters;

    #[test]
    fn example_one() {
        assert_eq!(reverse_only_letters("ab-cd".into()), "dc-ba".to_string());
    }

    #[test]
    fn example_two() {
        assert_eq!(
            reverse_only_letters("a-bC-dEf-ghIj".into()),
            "j-Ih-gfE-dCba".to_string()
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(reverse_only_letters("Test1ng-Leet=code-Q!".into()), "Qedo1ct-eeLg=ntse-T!".to_string());
    }
}
