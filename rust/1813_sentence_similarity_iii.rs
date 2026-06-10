/// LeetCode #1813 - Sentence Similarity III
fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
    let mut words1: Vec<&str> = sentence1.split_whitespace().collect();
    let mut words2: Vec<&str> = sentence2.split_whitespace().collect();
    if words1.len() < words2.len() {
        std::mem::swap(&mut words1, &mut words2);
    }
    let m = words1.len();
    let n = words2.len();
    let mut i = 0usize;
    while i < n && words1[i] == words2[i] {
        i += 1;
    }
    let mut j = 0usize;
    while j < n && words1[m - 1 - j] == words2[n - 1 - j] {
        j += 1;
    }
    i + j >= n
}

fn main() {
    println!(
        "{}",
        are_sentences_similar(
            "My name is Haley".into(),
            "My Haley".into(),
        )
    );
}

#[cfg(test)]
mod tests {
    use super::are_sentences_similar;

    #[test]
    fn example_one() {
        assert!(are_sentences_similar(
            "My name is Haley".into(),
            "My Haley".into(),
        ));
    }

    #[test]
    fn example_two() {
        assert!(!are_sentences_similar(
            "of".into(),
            "A lot of words".into(),
        ));
    }

    #[test]
    fn example_three() {
        assert!(are_sentences_similar(
            "Eating right now".into(),
            "Eating".into(),
        ));
    }
}
