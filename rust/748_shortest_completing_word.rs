/// LeetCode #748 - Shortest Completing Word
fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    let mut need = [0i32; 26];
    for c in license_plate.to_ascii_lowercase().chars() {
        if c.is_ascii_lowercase() {
            need[(c as u8 - b'a') as usize] += 1;
        }
    }
    let mut best: Option<String> = None;
    for w in words {
        let mut cnt = [0i32; 26];
        for c in w.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        let mut ok = true;
        for i in 0..26 {
            if cnt[i] < need[i] {
                ok = false;
                break;
            }
        }
        if ok {
            let take = match &best {
                None => true,
                Some(b) => w.len() < b.len(),
            };
            if take {
                best = Some(w);
            }
        }
    }
    best.unwrap()
}

fn main() {
    println!(
        "{}",
        shortest_completing_word(
            "1s3 PSt".into(),
            vec![
                "step".into(),
                "steps".into(),
                "stripe".into(),
                "stepple".into(),
            ],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_completing_word;

    #[test]
    fn example_one() {
        assert_eq!(
            shortest_completing_word(
                "1s3 PSt".into(),
                vec![
                    "step".into(),
                    "steps".into(),
                    "stripe".into(),
                    "stepple".into(),
                ],
            ),
            "steps"
        );
    }
}
