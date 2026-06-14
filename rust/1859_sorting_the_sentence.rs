/// LeetCode #1859 - Sorting the Sentence
fn sort_sentence(s: String) -> String {
    let ws: Vec<String> = s.split_whitespace().map(String::from).collect();
    let mut ans = vec![String::new(); ws.len()];
    for w in ws {
        let idx = (w.as_bytes()[w.len() - 1] - b'1') as usize;
        ans[idx] = w[..w.len() - 1].to_string();
    }
    ans.join(" ")
}

fn main() {
    println!("{}", sort_sentence("is2 sentence4 This1 a3".into()));
}

#[cfg(test)]
mod tests {
    use super::sort_sentence;

    #[test]
    fn example_one() {
        assert_eq!(
            sort_sentence("is2 sentence4 This1 a3".into()),
            "This is a sentence"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            sort_sentence("Myself2 Me1 I4 and3".into()),
            "Me Myself and I"
        );
    }
}
