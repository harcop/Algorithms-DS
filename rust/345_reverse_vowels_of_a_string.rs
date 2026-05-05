/// LeetCode #345 - Reverse Vowels of a String
fn reverse_vowels(s: String) -> String {
    let mut v: Vec<char> = s.chars().collect();
    let is_vowel = |c: char| "aeiouAEIOU".contains(c);
    if v.is_empty() {
        return s;
    }
    let (mut i, mut j) = (0usize, v.len() - 1);
    while i < j {
        while i < j && !is_vowel(v[i]) { i += 1; }
        while i < j && !is_vowel(v[j]) { j -= 1; }
        if i < j {
            v.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
    v.into_iter().collect()
}

fn main() {
    println!("{}", reverse_vowels("hello".into()));
}

#[cfg(test)]
mod tests {
    use super::reverse_vowels;

    #[test]
    fn example_one() {
        assert_eq!(reverse_vowels("hello".into()), "holle");
    }
}
