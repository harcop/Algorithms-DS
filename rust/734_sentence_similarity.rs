/// LeetCode #734 - Sentence Similarity
use std::collections::HashSet;

fn are_sentences_similar(
    sentence1: Vec<String>,
    sentence2: Vec<String>,
    similar_pairs: Vec<Vec<String>>,
) -> bool {
    if sentence1.len() != sentence2.len() {
        return false;
    }
    let mut set: HashSet<(String, String)> = HashSet::new();
    for p in similar_pairs {
        if p.len() == 2 {
            set.insert((p[0].clone(), p[1].clone()));
            set.insert((p[1].clone(), p[0].clone()));
        }
    }
    for i in 0..sentence1.len() {
        if sentence1[i] == sentence2[i] {
            continue;
        }
        if !set.contains(&(sentence1[i].clone(), sentence2[i].clone())) {
            return false;
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        are_sentences_similar(
            vec!["great".into(), "acting".into(), "skills".into()],
            vec!["fine".into(), "drama".into(), "talent".into()],
            vec![
                vec!["great".into(), "fine".into()],
                vec!["acting".into(), "drama".into()],
                vec!["skills".into(), "talent".into()],
            ],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::are_sentences_similar;

    #[test]
    fn example_one() {
        assert!(are_sentences_similar(
            vec!["great".into(), "acting".into(), "skills".into()],
            vec!["fine".into(), "drama".into(), "talent".into()],
            vec![
                vec!["great".into(), "fine".into()],
                vec!["acting".into(), "drama".into()],
                vec!["skills".into(), "talent".into()],
            ],
        ));
    }
}
