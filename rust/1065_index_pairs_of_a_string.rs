/// LeetCode #1065 - Index Pairs of a String
fn index_pairs(text: String, words: Vec<String>) -> Vec<Vec<i32>> {
    let bytes = text.as_bytes();
    let mut ans = Vec::new();
    for w in words {
        let pat = w.as_bytes();
        if pat.is_empty() {
            continue;
        }
        for i in 0..bytes.len() {
            if i + pat.len() <= bytes.len() && &bytes[i..i + pat.len()] == pat {
                ans.push(vec![i as i32, (i + pat.len() - 1) as i32]);
            }
        }
    }
    ans.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
    ans.dedup();
    ans
}

fn main() {
    println!(
        "{:?}",
        index_pairs(
            "thestoryofdeathandlife".into(),
            vec!["life".into(), "death".into(), "story".into()],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::index_pairs;

    #[test]
    fn example_one() {
        assert_eq!(
            index_pairs(
                "thestoryofdeathandlife".into(),
                vec!["life".into(), "death".into(), "story".into()],
            ),
            vec![vec![3, 7], vec![10, 14], vec![18, 21]]
        );
    }
}
