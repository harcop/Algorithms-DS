/// LeetCode #824 - Goat Latin
fn to_goat_latin(sentence: String) -> String {
    let vowels: &[u8] = b"aeiouAEIOU";
    sentence
        .split_whitespace()
        .enumerate()
        .map(|(i, word)| {
            let mut w: Vec<char> = word.chars().collect();
            if !vowels.contains(&(w[0] as u8)) {
                let c = w.remove(0);
                w.push(c);
            }
            w.push('m');
            w.push('a');
            for _ in 0..=i {
                w.push('a');
            }
            w.iter().collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    println!("{}", to_goat_latin("I speak Goat Latin".into()));
}

#[cfg(test)]
mod tests {
    use super::to_goat_latin;

    #[test]
    fn example_one() {
        assert_eq!(
            to_goat_latin("I speak Goat Latin".into()),
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
        );
    }
}
