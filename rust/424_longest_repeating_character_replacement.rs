/// LeetCode #424 - Longest Repeating Character Replacement
fn character_replacement(s: String, k: i32) -> i32 {
    let b = s.as_bytes();
    let k = k as usize;
    let mut freq = [0usize; 26];
    let mut lo = 0usize;
    let mut mx = 0usize;
    let mut best = 0usize;
    for hi in 0..b.len() {
        let c = (b[hi] - b'A') as usize;
        freq[c] += 1;
        mx = mx.max(freq[c]);
        while hi - lo + 1 - mx > k {
            let cl = (b[lo] - b'A') as usize;
            freq[cl] -= 1;
            lo += 1;
            mx = *freq.iter().max().unwrap();
        }
        best = best.max(hi - lo + 1);
    }
    best as i32
}

fn main() {
    println!("{}", character_replacement("ABAB".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::character_replacement;

    #[test]
    fn example_one() {
        assert_eq!(character_replacement("ABAB".into(), 2), 4);
    }
}
