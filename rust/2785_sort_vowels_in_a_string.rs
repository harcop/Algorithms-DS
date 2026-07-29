/// LeetCode #2785 - Sort Vowels in a String
fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn sort_vowels(s: &str) -> String {
    let mut vowels: Vec<char> = s.chars().filter(|&c| is_vowel(c)).collect();
    vowels.sort_unstable();
    let mut j = 0;
    s.chars()
        .map(|c| {
            if is_vowel(c) {
                let v = vowels[j];
                j += 1;
                v
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    println!("{}", sort_vowels("lEetcOde"));
}

#[cfg(test)]
mod tests {
    use super::sort_vowels;

    #[test]
    fn example_one() {
        assert_eq!(sort_vowels("lEetcOde"), "lEOtcede");
    }

    #[test]
    fn example_two() {
        assert_eq!(sort_vowels("lYmpH"), "lYmpH");
    }
}
