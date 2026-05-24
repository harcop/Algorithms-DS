/// LeetCode #1255 - Maximum Score Words Formed by Letters
fn max_score_words(scores: Vec<i32>, words: Vec<String>) -> i32 {
    let masks: Vec<(u32, i32)> = words
        .iter()
        .map(|w| {
            let mut m = 0u32;
            let mut sc = 0i32;
            for c in w.bytes() {
                let bit = 1u32 << (c - b'a');
                m |= bit;
                sc += scores[(c - b'a') as usize];
            }
            (m, sc)
        })
        .collect();
    let mut best = 0i32;
    fn dfs(i: usize, masks: &[(u32, i32)], used: u32, score: i32, best: &mut i32) {
        *best = (*best).max(score);
        for j in i..masks.len() {
            let (m, sc) = masks[j];
            if used & m == 0 {
                dfs(j + 1, masks, used | m, score + sc, best);
            }
        }
    }
    dfs(0, &masks, 0, 0, &mut best);
    best
}

fn main() {
    println!(
        "{}",
        max_score_words(
            vec![4, 3, 2],
            vec!["ccb".into(), "aab".into(), "bb".into(), "ca".into()]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_score_words;

    #[test]
    fn example_one() {
        assert_eq!(
            max_score_words(
                vec![4, 3, 2],
                vec!["ccb".into(), "aab".into(), "bb".into(), "ca".into()]
            ),
            6
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_score_words(
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec!["ad".into(), "bd".into(), "aaab".into(), "baa".into(), "badab".into()]
            ),
            8
        );
    }
}
