/// LeetCode #418 - Sentence Screen Fitting
fn words_typing(sentence: Vec<String>, rows: i32, cols: i32) -> i32 {
    let mut ring = sentence.join(" ");
    if ring.is_empty() {
        return 0;
    }
    ring.push(' ');
    let b = ring.into_bytes();
    let n = b.len();
    let cols = cols as usize;
    let rows = rows as usize;
    let mut pos = 0usize;
    for _ in 0..rows {
        pos += cols;
        if b[pos % n] == b' ' {
            pos += 1;
            continue;
        }
        while pos > 0 && b[(pos - 1) % n] != b' ' {
            pos -= 1;
        }
    }
    (pos / n) as i32
}

fn main() {
    println!(
        "{}",
        words_typing(vec!["hello".into(), "world".into()], 2, 8)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        assert_eq!(
            words_typing(vec!["hello".into(), "world".into()], 2, 8),
            1
        );
    }
}
