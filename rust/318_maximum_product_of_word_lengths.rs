/// LeetCode #318 - Maximum Product of Word Lengths
fn max_product(words: Vec<String>) -> i32 {
    let n = words.len();
    let mut masks = vec![0u32; n];
    let mut lens = vec![0i32; n];
    for (i, w) in words.iter().enumerate() {
        let mut m = 0u32;
        for b in w.bytes() {
            m |= 1 << (b - b'a') as u32;
        }
        masks[i] = m;
        lens[i] = w.len() as i32;
    }
    let mut best = 0i32;
    for i in 0..n {
        for j in i + 1..n {
            if masks[i] & masks[j] == 0 {
                best = best.max(lens[i] * lens[j]);
            }
        }
    }
    best
}

fn main() {
    println!(
        "{}",
        max_product(vec![
            "abcw".into(),
            "baz".into(),
            "foo".into(),
            "bar".into(),
            "xtfn".into(),
            "abcdef".into()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::max_product;

    #[test]
    fn example() {
        assert_eq!(
            max_product(vec![
                "abcw".into(),
                "baz".into(),
                "foo".into(),
                "bar".into(),
                "xtfn".into(),
                "abcdef".into()
            ]),
            16
        );
    }
}
