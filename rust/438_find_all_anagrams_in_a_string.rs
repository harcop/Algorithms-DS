/// LeetCode #438 - Find All Anagrams in a String
fn find_anagrams(s: String, p: String) -> Vec<i32> {
    if p.len() > s.len() {
        return vec![];
    }
    let mut need = [0i32; 26];
    for b in p.bytes() {
        need[(b - b'a') as usize] += 1;
    }
    let sb = s.as_bytes();
    let mut win = [0i32; 26];
    let mut out = vec![];
    for i in 0..sb.len() {
        win[(sb[i] - b'a') as usize] += 1;
        if i >= p.len() {
            win[(sb[i - p.len()] - b'a') as usize] -= 1;
        }
        if i + 1 >= p.len() && win == need {
            out.push((i + 1 - p.len()) as i32);
        }
    }
    out
}

fn main() {
    println!("{:?}", find_anagrams("cbaebabacd".into(), "abc".into()));
}

#[cfg(test)]
mod tests {
    use super::find_anagrams;

    #[test]
    fn example_one() {
        let mut v = find_anagrams("cbaebabacd".into(), "abc".into());
        v.sort_unstable();
        assert_eq!(v, vec![0, 6]);
    }
}
