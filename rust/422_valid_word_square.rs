/// LeetCode #422 - Valid Word Square
fn valid_word_square(words: Vec<String>) -> bool {
    let n = words.len();
    let b: Vec<&[u8]> = words.iter().map(|s| s.as_bytes()).collect();
    for i in 0..n {
        for j in 0..b[i].len() {
            if j >= n || i >= b[j].len() || b[i][j] != b[j][i] {
                return false;
            }
        }
    }
    for k in 0..n {
        if b[k].len() > n {
            return false;
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        valid_word_square(vec![
            "abcd".into(),
            "bnrt".into(),
            "crmy".into(),
            "dtye".into(),
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        assert!(valid_word_square(vec![
            "abcd".into(),
            "bnrt".into(),
            "crmy".into(),
            "dtye".into(),
        ]));
        assert!(valid_word_square(vec![
            "ball".into(),
            "area".into(),
            "lead".into(),
            "lady".into(),
        ]));
    }
}
