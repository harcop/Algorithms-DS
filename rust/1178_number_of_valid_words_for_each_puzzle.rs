/// LeetCode #1178 - Number of Valid Words for Each Puzzle
fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for w in words {
        let mut mask = 0i32;
        for ch in w.bytes() {
            mask |= 1 << (ch - b'a');
        }
        if mask.count_ones() > 7 {
            continue;
        }
        *cnt.entry(mask).or_insert(0) += 1;
    }
    puzzles
        .into_iter()
        .map(|p| {
            let letters: Vec<u8> = p.bytes().collect();
            let first = 1 << (letters[0] - b'a');
            let mut pm = 0i32;
            for &ch in &letters {
                pm |= 1 << (ch - b'a');
            }
            let mut sub = pm;
            let mut ans = 0i32;
            loop {
                if sub & first == first {
                    ans += cnt.get(&sub).copied().unwrap_or(0);
                }
                if sub == 0 {
                    break;
                }
                sub = (sub - 1) & pm;
            }
            ans
        })
        .collect()
}

fn main() {
    let words = vec!["aaaa".to_string(), "asas".to_string(), "able".to_string()];
    let puzzles = vec!["asla".to_string()];
    println!("{:?}", find_num_of_valid_words(words, puzzles));
}

#[cfg(test)]
mod tests {
    use super::find_num_of_valid_words;

    #[test]
    fn example_one() {
        assert_eq!(
            find_num_of_valid_words(
                vec![
                    "aaaa".to_string(),
                    "asas".to_string(),
                    "able".to_string(),
                    "ability".to_string(),
                    "actt".to_string(),
                    "actor".to_string(),
                    "access".to_string()
                ],
                vec![
                    "aboveyz".to_string(),
                    "abrodyz".to_string(),
                    "abslute".to_string(),
                    "absoryz".to_string(),
                    "actresz".to_string(),
                    "gaswxyz".to_string()
                ]
            ),
            vec![1, 1, 3, 2, 4, 0]
        );
    }
}
