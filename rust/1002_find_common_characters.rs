/// LeetCode #1002 - Find Common Characters
fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut freq = [i32::MAX; 26];
    for w in &words {
        let mut local = [0i32; 26];
        for c in w.bytes() {
            local[(c - b'a') as usize] += 1;
        }
        for i in 0..26 {
            freq[i] = freq[i].min(local[i]);
        }
    }
    let mut out = Vec::new();
    for i in 0..26 {
        for _ in 0..freq[i] {
            out.push(((b'a' + i as u8) as char).to_string());
        }
    }
    out
}

fn main() {
    println!("{:?}", common_chars(vec!["bella".into(), "label".into(), "roller".into()]));
}

#[cfg(test)]
mod tests {
    use super::common_chars;

    #[test]
    fn example_one() {
        let mut v = common_chars(vec!["bella".into(), "label".into(), "roller".into()]);
        v.sort();
        let mut exp = vec!["e".into(), "l".into(), "l".into()];
        exp.sort();
        assert_eq!(v, exp);
    }
}
